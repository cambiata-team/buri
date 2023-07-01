use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TargetName {
    Specific(String),
    // TODO: support recursive targets
}

#[derive(Debug, PartialEq)]
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
        format!("{}/BUILD", self.directories.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_target;

    #[test]
    fn test_target() {
        let target = Target {
            name: TargetName::Specific("test".to_string()),
            directories: vec!["foo".to_string(), "bar".to_string()],
        };
        assert_eq!(target.to_string(), "//foo/bar:test");
    }

    #[test]
    fn test_no_directories() {
        let target = Target {
            name: TargetName::Specific("test".to_string()),
            directories: vec![],
        };
        assert_eq!(target.to_string(), "//:test");
    }

    #[test]
    fn test_build_file_location() {
        let tests = [
            ["//foo", "foo/BUILD"],
            ["//foo:bar", "foo/BUILD"],
            ["//foo/bar", "foo/bar/BUILD"],
        ];
        for test in tests.iter() {
            let target = parse_target(test[0]).unwrap();
            assert_eq!(target.build_file_location(), test[1]);
        }
    }
}
