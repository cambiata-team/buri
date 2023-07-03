// Input can be either format: "v1.2.3" or "1.2.3" => normalize it to "1.2.3"
pub fn normalize_version(input: &str) -> &str {
    if let Some(stripped) = input.strip_prefix('v') {
        stripped
    } else {
        input
    }
}

pub fn is_valid_version(input: &str) -> bool {
    !input.is_empty() && !input.contains(char::is_whitespace)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn remove_the_v_from_semantic_versions() {
        assert_eq!(normalize_version("v1.0.42"), "1.0.42");
    }

    #[test]
    fn semantic_versions_remain_unchanged() {
        assert_eq!(normalize_version("1.0.42"), "1.0.42");
    }

    #[test]
    fn normalize_version_works_with_date_based_versions_with_v() {
        assert_eq!(normalize_version("v2023-07-03"), "2023-07-03");
    }

    #[test]
    fn normalize_version_works_with_date_based_versions() {
        assert_eq!(normalize_version("2023-07-03"), "2023-07-03");
    }

    #[test]
    fn empty_versions_are_not_valid() {
        assert!(!is_valid_version(""));
    }

    #[test]
    fn versions_with_any_whitespace_are_not_valid() {
        assert!(!is_valid_version(" "));
        assert!(!is_valid_version("1 2 3"));
        assert!(!is_valid_version("11.\t.3"));
    }
}
