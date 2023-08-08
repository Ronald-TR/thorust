use serde::{Serialize, Deserialize};
use anyhow::Result;

use crate::{traits::Manifest, entities::{conversions::{new_uuidv4, to_grpcurl_command}, graph::{TestNode, TestExecutable}, enums::TestStatus}};

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
    #[serde(default = "new_uuidv4")]
    pub id: String,
    #[serde(default = "Vec::new")]
    pub depends_on: Vec<String>,
    pub description: String,
    pub method: String,
    pub proto: String,
    pub body: String,
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

impl MGrpcFile {
    /// Format all test `id` and depends_on ids as <service>.<test_id>
    pub fn format_test_ids(&mut self) {
        let service_names = self
            .services
            .iter()
            .map(|s| s.name.clone())
            .collect::<Vec<String>>();
        for service in self.services.iter_mut() {
            for test in service.tests.iter_mut() {
                // Format the test_id as <service>.<test_id>
                test.id = format!("{}.{}", service.name, test.id);
                // Format the test_id depends_on as <service>.<test_id> as well,
                // If the first split element is not a service name, it means that the dependency id
                // is from a test for the same service, so we add the service name to it.
                test.depends_on.iter_mut().for_each(|dep| {
                    let mut slice = dep.split('.');
                    let service_name = slice.nth(0).unwrap_or_default();
                    if !(service_names.contains(&service_name.to_owned())) {
                        *dep = format!("{}.{}", service.name, dep)
                    }
                });
            }
        }
    }
}

impl Manifest for MGrpcFile {
    fn normalize(&mut self) -> Result<()> {
        self.format_test_ids();
        Ok(())
    }
    fn as_test_nodes(&self) -> Result<Vec<TestNode>> {
        let mut nodes: Vec<TestNode> = Vec::new();
        let mut index: u32 = 0;
        for service in self.services.iter() {
            for test in service.tests.iter() {
                let command = to_grpcurl_command(
                    &test.headers,
                    &test.body,
                    &test.proto,
                    &service.address,
                    &test.method
                );
                nodes.push(TestNode {
                    id: test.id.clone(),
                    index,
                    status: vec![TestStatus::NotStarted],
                    depends_on: test.depends_on.clone(),
                    executable: TestExecutable {
                        name: test.name.clone(),
                        service: service.name.clone(),
                        command,
                        description: test.description.clone(),
                        id: test.id.clone(),
                        output: None,
                    },
                });
                index += 1;
            }
        }
        Ok(nodes)
    }
}
