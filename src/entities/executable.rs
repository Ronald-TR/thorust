use anyhow::Result;
use tokio::process::Command;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TestExecutable {
    pub id: String,
    pub service: String,
    pub name: String,
    pub command: String,
    pub output: Option<String>,
}

impl TestExecutable {
    pub async fn call(&self) -> Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .output()
            .await?;
        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "The test '{}' failed with exit code: {}",
                self.name,
                output.status
            ));
        }
        Ok(String::from_utf8(output.stdout).unwrap_or_default())
    }
}
