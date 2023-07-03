use serde::{Deserialize, Serialize};

// Do not change without supplying a migration script.
// This will lead to incompatibilities between versions.
pub const WORKSPACE_FILE_NAME: &str = "WORKSPACE.toml";

#[derive(Deserialize, Serialize, Default)]
pub struct WorkspaceFile {
    pub name: Option<String>,
}

impl WorkspaceFile {
    pub fn new() -> Self {
        Self { name: None }
    }
}
