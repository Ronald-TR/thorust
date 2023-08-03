use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MGrpcFile {
    pub services: Vec<Service>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    pub name: String,
    // #[serde(deserialize_with = "deserialize_with_env_state")]
    pub address: String,
    pub tests: Vec<TestUnit>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestUnit {
    pub name: String,
    pub description: String,
    pub method: String,
    pub proto: String,
    pub body: serde_json::Value,
    pub headers: Option<Vec<String>>,
    pub expected: Option<ReqSpec>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ReqSpec {
    pub status: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct CurlErrorInner {
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct CurlError {
    #[serde(rename = "ERROR")]
    pub error: CurlErrorInner,
}