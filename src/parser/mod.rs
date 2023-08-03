use anyhow::Result;
use petgraph::prelude::DiGraph;
use std::fs::File;

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
#[derive(Debug, PartialEq)]
pub enum ParserType {
    Yaml,
    Json,
}

pub fn parse_file(fp: &str, normalize: bool) -> Result<RootFile> {
    let parser_type = ParserType::from_filepath(fp);
    let mut root: RootFile = match parser_type {
        ParserType::Yaml => serde_yaml::from_reader(File::open(&fp)?)?,
        ParserType::Json => serde_json::from_reader(File::open(&fp)?)?,
    };
    if normalize {
        root.format_test_ids();
        root.checks_depends_on()?;
    }
    Ok(root)
}

/// Like parse(), but for a entiry directory
pub fn parse_dir(dir: &str, normalize: bool) -> Result<RootFile> {
    let mut services = vec![];
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let mut file = parse_file(path.to_str().unwrap(), normalize)?;
            services.append(&mut file.services);
        }
    }
    let root = RootFile { services };
    Ok(root)
}

/// Standard parser for the manifest files
/// The files format can be yaml or json.
///
/// If the path is a directory, it will parse all the files inside it into one.
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
    let mut root = match std::path::Path::new(fp).is_dir() {
        true => parse_dir(fp, false),
        false => parse_file(fp, false),
    }?;
    root.format_test_ids();
    root.checks_depends_on()?;
    Ok(root)
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
