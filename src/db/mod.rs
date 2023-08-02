use std::rc::Rc;

use rusqlite::{Connection, Result};

use crate::{
    entities::{
        graph::TestNode,
        storage::{DbGraph, DbNode, NodeHistory, ProcessedHistory},
    },
    traits::Storage,
};

use self::queries::{
    ALL_HISTORY_WITH_DURATION_BETWEEN_STATUS, HISTORY_WITH_DURATION_BETWEEN_STATUS,
};
pub mod queries;

#[derive(Debug)]
pub struct SqliteStorage {}

impl SqliteStorage {
    // Create a new and empty instance of the storage
    pub fn new() -> Self {
        let conn = Connection::open("./db").unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS nodes (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL,
                description     TEXT NOT NULL,
                service         TEXT NOT NULL,
                test_id         TEXT NOT NULL
            )",
            (),
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS node_history (
                id              INTEGER PRIMARY KEY,
                status          TEXT NOT NULL,
                node            INTEGER NOT NULL,
                created_at      TIMESTAMP DEFAULT(STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')),
                data            TEXT DEFAULT '',
                FOREIGN KEY(node) REFERENCES nodes(id)
            )",
            (),
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS graph (
                id              INTEGER PRIMARY KEY,
                dot             TEXT NOT NULL,
                created_at      TIMESTAMP DEFAULT(STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW'))
              )",
            (),
        )
        .unwrap();
        Self {}
    }
    pub fn conn(&self) -> Connection {
        Connection::open("./db").unwrap()
    }
}

#[async_trait::async_trait]
impl Storage for SqliteStorage {
    fn insert_test_node(&self, node: &TestNode) {
        let dbnode = node.clone().into();
        let status = node.last_status();
        let node_id = self.insert_node(dbnode);
        self.insert_node_history(
            &status.to_string(),
            node_id,
            &node.executable.output.clone().unwrap_or_default(),
        );
    }

    fn insert_node(&self, node: DbNode) -> i64 {
        let conn = self.conn();
        conn.execute(
            "INSERT INTO nodes (id, test_id, name, description, service)
                VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                node.id,
                node.test_id,
                node.name,
                node.description,
                node.service,
            ),
        )
        .unwrap();
        conn.last_insert_rowid()
    }

    fn insert_node_history(&self, status: &str, node_id: i64, data: &str) -> i64 {
        let conn = self.conn();
        conn.execute(
            "INSERT INTO node_history (status, node, data) VALUES (?1, ?2, ?3)",
            (status, node_id, data),
        )
        .unwrap();
        conn.last_insert_rowid()
    }

    fn insert_dot(&self, dot: &str) -> i64 {
        let conn = self.conn();
        conn.execute("INSERT INTO graph (dot) VALUES (?1)", (dot,))
            .unwrap();
        conn.last_insert_rowid()
    }

    fn get_nodes(&self, ids: &[i32]) -> Result<Vec<DbNode>> {
        let conn = self.conn();
        rusqlite::vtab::array::load_module(&conn)?;
        let values = Rc::new(
            ids.iter()
                .copied()
                .map(rusqlite::types::Value::from)
                .collect::<Vec<rusqlite::types::Value>>(),
        );
        let mut stmt = conn.prepare(
            "SELECT id, test_id, name, description, service FROM nodes WHERE id IN rarray(?1) ORDER BY id ASC"
        )?;
        let node_iter = stmt.query_map([values], |row| {
            Ok(DbNode {
                id: row.get(0)?,
                test_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                service: row.get(4)?,
            })
        })?;
        node_iter.collect()
    }
    fn get_all_nodes(&self) -> Result<Vec<DbNode>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT id, test_id, name, description, service FROM nodes ORDER BY id ASC"
        )?;
        let node_iter = stmt.query_map([], |row| {
            Ok(DbNode {
                id: row.get(0)?,
                test_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                service: row.get(4)?,
            })
        })?;
        node_iter.collect()
    }

    fn get_node_history(&self, node_id: i32) -> Result<Vec<NodeHistory>> {
        let conn = self.conn();
        let mut stmt =
            conn.prepare("SELECT id, status, node, data, created_at FROM node_history WHERE node = ?1 ORDER BY created_at ASC")?;
        let history_iter = stmt.query_map([node_id], |row| {
            Ok(NodeHistory {
                id: row.get(0)?,
                status: row.get(1)?,
                node: row.get(2)?,
                data: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        history_iter.collect()
    }

    fn get_processed_node_history(&self, node_id: i32) -> Result<Vec<ProcessedHistory>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(HISTORY_WITH_DURATION_BETWEEN_STATUS)?;
        let history_iter = stmt
            .query_map([node_id], |row| {
                Ok(ProcessedHistory {
                    node: row.get(0)?,
                    from_status: row.get(1).unwrap_or_default(),
                    to_status: row.get(2).unwrap_or_default(),
                    from_created_at: row.get(3)?,
                    to_created_at: row.get(4).unwrap_or_default(),
                    duration_millis: row.get(5).unwrap_or_default(),
                })
            })?
            .collect::<Result<Vec<ProcessedHistory>>>()?;
        // filter out empty to_status
        // it means that the from_status is the last status provided.
        Ok(history_iter
            .into_iter()
            .filter(|h| !h.to_status.is_empty())
            .collect())
    }

    fn get_all_processed_node_history(&self) -> Result<Vec<ProcessedHistory>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(ALL_HISTORY_WITH_DURATION_BETWEEN_STATUS)?;
        let history_iter = stmt
            .query_map([], |row| {
                Ok(ProcessedHistory {
                    node: row.get(0)?,
                    from_status: row.get(1).unwrap_or_default(),
                    to_status: row.get(2).unwrap_or_default(),
                    from_created_at: row.get(3)?,
                    to_created_at: row.get(4).unwrap_or_default(),
                    duration_millis: row.get(5).unwrap_or_default(),
                })
            })?
            .collect::<Result<Vec<ProcessedHistory>>>()?;
        // filter out empty to_status
        // it means that the from_status is the last status provided.
        Ok(history_iter
            .into_iter()
            .filter(|h| !h.to_status.is_empty())
            .collect())
    }

    fn get_dots(&self) -> Result<Vec<DbGraph>> {
        let conn = self.conn();
        let mut stmt = conn.prepare("SELECT id, dot, created_at FROM graph")?;
        let graph_iter = stmt.query_map([], |row| {
            Ok(DbGraph {
                id: row.get(0)?,
                dot: row.get(1)?,
                created_at: row.get(2)?,
            })
        })?;
        graph_iter.collect()
    }
    fn insert_test_nodes(&self, nodes: Vec<&TestNode>) {
        for node in nodes {
            self.insert_test_node(node);
        }
    }
}
