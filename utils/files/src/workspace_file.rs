use serde::{Deserialize, Serialize};

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
