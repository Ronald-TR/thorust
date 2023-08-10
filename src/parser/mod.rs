use anyhow::{anyhow, Result};
use petgraph::prelude::DiGraph;
use regex::Regex;
use serde::de;
use std::{fs::File, path::Path, str::FromStr};

use crate::{
    entities::{
        enums::{ExtType, ManifestKind},
        graph::TestNode,
        manifests::BaseManifest,
    },
    traits::Manifest,
};

pub fn orphan_nodes<'a>(graph: &DiGraph<&'a TestNode, &'a usize>) -> Vec<&'a TestNode> {
    let mut orphans = Vec::new();
    for node in graph.externals(petgraph::Direction::Incoming) {
        let n = *graph.node_weight(node).unwrap();
        orphans.push(n);
    }
    orphans
}

#[derive(Debug, PartialEq)]
pub struct ParserInfo {
    pub filename: String,
    pub parser: ManifestKind,
    pub ext: ExtType,
}

fn serialize_from_ext<T>(path: &str, ext: ExtType) -> Result<T>
where
    T: de::DeserializeOwned,
{
    match ext {
        ExtType::Json => Ok(serde_json::from_reader(File::open(path)?)?),
        ExtType::Yaml => Ok(serde_yaml::from_reader(File::open(path)?)?),
    }
}

/// Parse manifest file based on ParserInfo (extension, type, etc) extracted from file path.
pub fn parse_file(fp: &str, normalize: bool) -> Result<BaseManifest> {
    let parser_info = ParserInfo::new(fp)?;
    let (scripts, grpc) = match parser_info.parser {
        ManifestKind::Scripts => (serialize_from_ext(fp, parser_info.ext)?, None),
        ManifestKind::Grpc => (None, serialize_from_ext(fp, parser_info.ext)?),
    };
    let mut root = BaseManifest { scripts, grpc };
    if normalize {
        root.normalize()?;
    }
    Ok(root)
}

/// Like parse(), but for a entiry directory
///
/// Append all manifests into the dir on the same BaseManifest object.
pub fn parse_dir(dir: &str, normalize: bool) -> Result<BaseManifest> {
    let mut root_files = vec![];
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file = parse_file(path.to_str().unwrap(), normalize)?;
            root_files.push(file);
        }
    }
    let root = root_files.into_iter().sum();
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
///   let content = parse("manifests_example/example.scripts.yaml")?;
///   println!("Content: {:?}", content);
///  Ok(())
/// }
/// ```
pub fn parse(fp: &str) -> Result<BaseManifest> {
    let mut root = match std::path::Path::new(fp).is_dir() {
        true => parse_dir(fp, false),
        false => parse_file(fp, false),
    }?;
    let _ = root.normalize();
    Ok(root)
}

/// ParserInfo implementation
impl ParserInfo {
    /// from_filepath
    /// Create a ParserInfo based on the file path
    ///
    /// # Arguments
    ///
    /// * `fp` - A string slice that holds the file path
    ///
    /// # Returns
    ///
    /// * `ParserInfo` - A ParserInfo struct
    ///
    /// # Remarks
    ///
    /// * If the file name pattern is not supported, it will return an Error
    ///
    /// # Example
    ///
    /// ```
    /// use thorust::parser::ParserInfo;
    /// use thorust::entities::enums::{ManifestKind, ExtType};
    ///
    /// let parser_info = ParserInfo::new("foo/bar.scripts.yaml").unwrap();
    /// assert_eq!(parser_info.parser, ManifestKind::Scripts);
    /// assert_eq!(parser_info.ext, ExtType::Yaml);
    /// ```
    pub fn new(fp: &str) -> Result<ParserInfo> {
        let path = Path::new(fp);
        let filename = path
            .file_name()
            .ok_or_else(|| anyhow!("Could not find filename for path {}", fp))?;
        let pattern = r"(?P<filename>[^.]+)\.(?P<parser_type>[^.]+)\.(?P<ext_type>[^.]+)";
        let regex = Regex::new(pattern)?;

        match regex.captures(filename.to_str().unwrap_or_default()) {
            Some(captures) => {
                let filename = captures.name("filename").unwrap().as_str();
                let parser_type = captures.name("parser_type").unwrap().as_str();
                let ext_type = captures.name("ext_type").unwrap().as_str();
                Ok(ParserInfo {
                    filename: filename.to_owned(),
                    parser: ManifestKind::from_str(parser_type)?,
                    ext: ExtType::from_str(ext_type)?,
                })
            }
            None => Err(anyhow!("No match found for file {}.", fp)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        entities::enums::{ExtType, ManifestKind},
        parser::ParserInfo,
    };

    #[test]
    fn assert_parser_ext_serialization_should_pass() {
        assert_eq!(ExtType::from_str("yaml").unwrap(), ExtType::Yaml);
        assert_eq!(ExtType::from_str("json").unwrap(), ExtType::Json);
    }

    #[test]
    fn assert_parser_ext_serialization_should_not_pass() {
        assert!(ExtType::from_str("YAML").is_err());
        assert!(ExtType::from_str("YaMl").is_err());
        assert!(ExtType::from_str("JSON").is_err());
        assert!(ExtType::from_str("JsOn").is_err());
    }

    #[test]
    fn assert_parser_info_regex_success() {
        let parser_info = ParserInfo::new("foo/bar.scripts.yaml").unwrap();

        assert_eq!(parser_info.filename, "bar".to_string());
        assert_eq!(parser_info.parser, ManifestKind::Scripts);
        assert_eq!(parser_info.ext, ExtType::Yaml);
    }

    #[test]
    fn assert_parser_info_regex_wrong_file_format() {
        assert!(ParserInfo::new("foo/bar.extra_separator_unallowed.scripts.yaml").is_err());
        assert!(ParserInfo::new("foo/bar.wrong_type.yaml").is_err());
        assert!(ParserInfo::new("foo/bar.scripts.wrong_extension").is_err());
    }
}
