use serde::{Deserialize, Serialize};

// Do not change without supplying a migration script.
// This will lead to incompatibilities between versions.
pub const BUILD_FILE_NAME: &str = "BUILD.toml";

#[derive(Deserialize, Serialize)]
pub struct BuildFile {
    pub library: Option<Vec<Library>>,
}

#[derive(Deserialize, Serialize)]
pub struct Library {
    /// name of the library target
    pub name: String,
    /// all source files in this library
    pub files: Option<Vec<String>>,
    /// any targets this library depends on (including external deps)
    pub dependencies: Option<Vec<String>>,
    /// targets that depend on this target
    pub dependents: Option<Vec<String>>,
}
