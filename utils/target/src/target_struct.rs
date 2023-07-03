use files::build_file::BUILD_FILE_NAME;
use std::fmt;

pub(crate) type Index = u16;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TargetName {
    /// Target name starts Index characters from the end of the raw string
    /// not including the colon.
    Specific(Index),
    // TODO: support recursive targets
}

#[derive(Debug, PartialEq, Clone)]
// Everything is saved as indices to reduce memory and heap allocations.
pub struct Target {
    pub(crate) name: TargetName,
    pub(crate) directories_end: Index,
    pub(crate) raw_text: String,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            &self.raw_text[..self.directories_end as usize],
            self.name()
        )
    }
}

impl Target {
    pub fn build_file_location(&self) -> String {
        format!("{}/{}", &self.get_directories(), BUILD_FILE_NAME)
    }

    pub fn get_directories(&self) -> &str {
        println!("{}", self.raw_text);
        &self.raw_text[..self.directories_end as usize]
    }

    pub fn name(&self) -> &str {
        match &self.name {
            TargetName::Specific(index) => &self.raw_text[*index as usize..],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::parse_target;

    #[test]
    fn test_target() {
        let target = parse_target("foo/bar:test").unwrap();
        assert_eq!(target.to_string(), "foo/bar:test");
    }

    #[test]
    fn test_no_directories() {
        let target = parse_target(":test").unwrap();
        assert_eq!(target.to_string(), ":test");
    }

    #[test]
    fn test_build_file_location() {
        let tests = [
            ["foo", "foo/BUILD.toml"],
            ["foo:bar", "foo/BUILD.toml"],
            ["foo/bar", "foo/bar/BUILD.toml"],
        ];
        for test in tests.iter() {
            let target = parse_target(test[0]).unwrap();
            assert_eq!(target.build_file_location(), test[1]);
        }
    }
}
