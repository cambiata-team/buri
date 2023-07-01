use serde::{Deserialize, Serialize};

pub const BUILD_FILE_NAME: &str = "BUILD.toml";

#[derive(Deserialize, Serialize)]
pub struct BuildFile {
    pub library: Vec<Library>,
}

#[derive(Deserialize, Serialize)]
pub struct Library {
    /// name of the library target
    pub name: String,
    /// all source files in this library
    pub files: Vec<String>,
    /// any targets this library depends on (including external deps)
    pub dependencies: Vec<String>,
    /// targets that depend on this target
    pub dependents: Vec<String>,
}
