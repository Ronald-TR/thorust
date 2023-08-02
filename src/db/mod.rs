use rusqlite::{Connection, Result};

use crate::{
    entities::{enums::TestStatus, graph::TestNode},
    traits::Storage,
};

#[derive(Debug)]
pub struct SqliteStorage {}

#[derive(Clone)]
pub struct DbNode {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub service: String,
    pub response: String,
}

#[derive(Clone, Default)]
pub struct DbGraph {
    pub id: i32,
    pub dot: String,
    pub created_at: String,
}

#[derive(Clone)]
pub struct NodeHistory {
    pub id: i32,
    pub status: String,
    pub node: i32,
    pub created_at: String,
}

impl SqliteStorage {
    // Create a new and empty instance of the storage
    pub fn new() -> Self {
        let conn = Connection::open("./db").unwrap();
        // let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS nodes (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL,
                description     TEXT NOT NULL,
                service         TEXT NOT NULL,
                response        TEXT DEFAULT ''
            )",
            (),
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS node_history (
                id              INTEGER PRIMARY KEY,
                status          TEXT NOT NULL,
                node            INTEGER NOT NULL,
                created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(node) REFERENCES nodes(id)
            )",
            (),
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS graph (
                id              INTEGER PRIMARY KEY,
                dot             TEXT NOT NULL,
                created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP
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
    fn insert_node_with_status(&self, node: DbNode, status: &TestStatus) {
        let node_id = self.insert_node(node);
        self.insert_node_history(&status.to_string(), node_id);
    }

    fn insert_node(&self, node: DbNode) -> i64 {
        let conn = self.conn();
        conn.execute(
            "INSERT INTO nodes (id, name, description, service)
                VALUES (?1, ?2, ?3, ?4)",
            (node.id, node.name, node.description, node.service),
        )
        .unwrap();
        conn.last_insert_rowid()
    }

    fn insert_node_history(&self, status: &str, node_id: i64) -> i64 {
        let conn = self.conn();
        conn.execute(
            "INSERT INTO node_history (status, node) VALUES (?1, ?2)",
            (status, node_id),
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

    fn get_nodes(&self) -> Result<Vec<DbNode>> {
        let conn = self.conn();
        let mut stmt =
            conn.prepare("SELECT id, name, description, service, response FROM nodes ORDER BY index ASC")?;
        let node_iter = stmt.query_map([], |row| {
            Ok(DbNode {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                service: row.get(3)?,
                response: row.get(4)?,
            })
        })?;
        node_iter.collect()
    }

    fn get_node_history(&self, node_id: i32) -> Result<Vec<NodeHistory>> {
        let conn = self.conn();
        let mut stmt =
            conn.prepare("SELECT id, status, node, created_at FROM node_history WHERE node = ?1 ORDER BY created_at ASC")?;
        let history_iter = stmt.query_map([node_id], |row| {
            Ok(NodeHistory {
                id: row.get(0)?,
                status: row.get(1)?,
                node: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;
        history_iter.collect()
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
    fn insert_nodes_from(&self, nodes: Vec<&TestNode>) {
        let dbnodes = nodes
            .into_iter()
            .map(|n| {
                (
                    n.clone().into(),
                    n.status
                        .last()
                        .unwrap_or_else(|| &TestStatus::NotStarted)
                        .clone(),
                )
            })
            .collect::<Vec<(DbNode, TestStatus)>>();
        for (node, status) in dbnodes {
            self.insert_node_with_status(node, &status);
        }
    }
}
