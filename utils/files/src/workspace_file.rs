use serde::Deserialize;

pub const WORKSPACE_FILE_NAME: &str = "WORKSPACE.toml";

#[derive(Deserialize)]
pub struct WorkspaceFile {
    pub name: Option<String>,
}
