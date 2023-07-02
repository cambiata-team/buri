use std::fmt;

use files::build_file::BUILD_FILE_NAME;

#[derive(Debug, PartialEq, Clone)]
pub enum TargetName {
    Specific(String),
    // TODO: support recursive targets
}

impl fmt::Display for TargetName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TargetName::Specific(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Target {
    pub name: TargetName,
    pub directories: Vec<String>,
    // TODO: support relative targets
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "//{}:{}",
            self.directories.join("/"),
            match &self.name {
                TargetName::Specific(name) => name,
            }
        )
    }
}

impl Target {
    pub fn build_file_location(&self) -> String {
        format!("{}/{}", self.directories.join("/"), BUILD_FILE_NAME)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::parse_target;

    #[test]
    fn test_target() {
        let target = parse_target("//foo/bar:test").unwrap();
        assert_eq!(target.to_string(), "//foo/bar:test");
    }

    #[test]
    fn test_no_directories() {
        let target = parse_target("//:test").unwrap();
        assert_eq!(target.to_string(), "//:test");
    }

    #[test]
    fn test_build_file_location() {
        let tests = [
            ["//foo", "foo/BUILD.toml"],
            ["//foo:bar", "foo/BUILD.toml"],
            ["//foo/bar", "foo/bar/BUILD.toml"],
        ];
        for test in tests.iter() {
            let target = parse_target(test[0]).unwrap();
            assert_eq!(target.build_file_location(), test[1]);
        }
    }
}
