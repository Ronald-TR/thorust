use anyhow::Result;
use serde::{Deserialize, Serialize};

fn new_uuidv4() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MScriptFile {
    pub services: Vec<Service>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    pub name: String,
    pub tests: Vec<TestUnit>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestUnit {
    pub name: String,
    #[serde(default = "new_uuidv4")]
    pub id: String,
    #[serde(default = "Vec::new")]
    pub depends_on: Vec<String>,
    pub command: String,
    pub description: String,
}

impl MScriptFile {
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
    /// Checks if the test dependencies has ids that are previously defined.
    pub fn checks_depends_on(&self) -> Result<()> {
        let mut ids: Vec<String> = Vec::new();
        for service in self.services.iter() {
            for test in service.tests.iter() {
                ids.push(test.id.clone());
            }
        }
        for service in self.services.iter() {
            for test in service.tests.iter() {
                for dep in test.depends_on.iter() {
                    if !ids.contains(dep) {
                        return Err(anyhow::anyhow!("The test id '{}' does not exist!", dep));
                    }
                }
            }
        }
        Ok(())
    }
    pub fn get_test(&self, test_id: &str) -> Option<&TestUnit> {
        for service in self.services.iter() {
            for test in service.tests.iter() {
                if test.id == test_id {
                    return Some(test);
                }
            }
        }
        None
    }
}
