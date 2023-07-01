use serde::Deserialize;

#[derive(Deserialize)]
pub struct WorkspaceFile {
    pub name: Option<String>,
}
