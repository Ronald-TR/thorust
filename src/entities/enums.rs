use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString)]
pub enum TestStatus {
    // When the test is waiting to be started
    NotStarted,
    // When the test is running
    Running,
    // When the test has finished successfully
    Completed,
    // When the test has failed
    Failed,
    // When the test has been skipped
    // Commonly is used when the test depends on another test that has failed
    Skipped,
}
