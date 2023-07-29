use std::fs::File;
use anyhow::Result;
use petgraph::prelude::DiGraph;

use crate::entities::{graph::TestNode, manifest::RootFile};

pub fn orphan_nodes<'a>(graph: &DiGraph<&'a TestNode, &'a usize>) -> Vec<&'a TestNode> {
    let mut orphans = Vec::new();
    for node in graph.externals(petgraph::Direction::Incoming) {
        let n = *graph.node_weight(node).unwrap();
        orphans.push(n);
    }
    orphans
}

/// Enum ParserType
/// Tt defines which parser to use
pub enum ParserType {
    Yaml,
    Json,
}

/// Standard parser for the manifest files
/// The format can be yaml or json
/// 
/// # Example
/// 
/// ```
/// use anyhow::Result;
/// use thorust::parser::parse;
/// 
/// fn main() -> Result<()> {
///   let content = parse("example.yaml")?;
///   println!("Content: {:?}", content);
///  Ok(())
/// }
/// ```
pub fn parse(fp: &str) -> Result<RootFile> {
    let parser_type = ParserType::from_filepath(fp);
    let mut content: RootFile = match parser_type {
        ParserType::Yaml => serde_yaml::from_reader(File::open(&fp)?)?,
        ParserType::Json => serde_json::from_reader(File::open(&fp)?)?,
    };
    content.format_test_ids();
    content.checks_depends_on()?;
    Ok(content)
}

/// ParserType implementation
impl ParserType {
    /// from_filepath
    /// Create a ParserType based in the file extension
    /// 
    /// # Arguments
    /// 
    /// * `fp` - A string slice that holds the file path
    /// 
    /// # Returns
    /// 
    /// * `ParserType` - A ParserType enum
    /// 
    /// # Remarks
    /// 
    /// * If the file extension is not supported, it will return the default ParserType::Yaml
    /// 
    /// # Example
    /// 
    /// ```
    /// use thorust::parser::ParserType;
    /// 
    /// let parser_type = ParserType::from_filepath("foo/bar.yaml");
    /// assert_eq!(parser_type, ParserType::Yaml);
    /// ```
    pub fn from_filepath(fp: &str) -> ParserType {
        match fp {
            "yaml" => ParserType::Yaml,
            "json" => ParserType::Json,
            _ => ParserType::Yaml,
        }
    }
}
