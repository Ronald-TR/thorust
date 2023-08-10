use crate::entities::graph::TestExecutable;
use anyhow::Result;
use tokio::process::Command;

pub async fn grpc_call(_test: &mut TestExecutable) -> Result<()> {
    todo!()
}

pub async fn scripts_call(test: &mut TestExecutable) -> Result<()> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&test.command)
        .output()
        .await?;
    test.exit_code = output.status.code();
    test.output = match output.status.success() {
        true => String::from_utf8(output.stdout)
            .map(Option::Some)
            .unwrap_or(None),
        false => String::from_utf8(output.stderr)
            .map(Option::Some)
            .unwrap_or(None),
    };

    output.status.success().then_some(()).ok_or_else(|| {
        anyhow::anyhow!(
            "The test '{}' failed with exit code: {} and message: {:?}",
            test.name,
            output.status,
            test.output
        )
    })
}
