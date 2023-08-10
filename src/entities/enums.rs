use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, Serialize, Deserialize)]
pub enum TestStatus {
    // When the test is waiting to be started
    NotStarted,
    // When the test is running
    Running,
    // When the test has finished successfully
    Completed,
    // When the test failed during execution
    Failed,
    // When the test fails during the response checks specified
    AssertionFailed,
    // When the test has been skipped
    // Commonly is used when the test depends on another test that has failed
    Skipped,
}

/// Enum ManifestKind,
/// defines which manifest parser to use
#[derive(Debug, Clone, Default, PartialEq, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum ManifestKind {
    Grpc,
    #[default]
    Scripts,
}

/// Enum ExtType
#[derive(Debug, Clone, PartialEq, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum ExtType {
    Json,
    Yaml,
}
