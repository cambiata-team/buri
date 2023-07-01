use target::Target;

#[derive(Debug, PartialEq)]
pub struct TargetFiles {
    pub target: Target,
    pub files: Vec<String>,
}
