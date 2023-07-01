use serde::Deserialize;

#[derive(Deserialize)]
pub struct BuildFile {
    pub library: Vec<Library>,
}

#[derive(Deserialize)]
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
