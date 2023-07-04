use crate::context::Context;
use files::cli_config::{CliConfig, CLI_CONFIG_FILE_NAME};
use vfs::VfsPath;

pub fn get_configured_thor_version(context: &Context) -> Option<String> {
    let config_file = context.root.join(CLI_CONFIG_FILE_NAME).ok()?;
    if !config_file.exists().ok()? || !config_file.is_file().ok()? {
        return None;
    }
    let config_file_contents = config_file.read_to_string().ok()?;
    let version_file = CliConfig::from(&config_file_contents).ok()?;
    version_file.get_version()
}

pub fn get_thor_binary_path(context: &Context, version: String) -> Option<VfsPath> {
    context.cache_dir.join(format!("thor@{version}")).ok()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_none_if_config_file_does_not_exist() {
        let context = Context::test();
        assert_eq!(get_configured_thor_version(&context), None);
    }

    #[test]
    fn returns_none_if_config_file_is_directory() {
        let context = Context::test();
        context
            .root
            .join(CLI_CONFIG_FILE_NAME)
            .unwrap()
            .create_dir_all()
            .unwrap();
        assert_eq!(get_configured_thor_version(&context), None);
    }

    #[test]
    fn returns_none_if_config_file_is_empty() {
        let context = Context::test();
        context
            .root
            .join(CLI_CONFIG_FILE_NAME)
            .unwrap()
            .create_file()
            .unwrap();
        assert_eq!(get_configured_thor_version(&context), None);
    }

    #[test]
    fn return_version_if_config_file_has_version() {
        let context = Context::test();
        context
            .root
            .join(CLI_CONFIG_FILE_NAME)
            .unwrap()
            .create_file()
            .unwrap()
            .write_all(b"buri_version=\"0.4.0\"")
            .unwrap();
        assert_eq!(
            get_configured_thor_version(&context),
            Some("0.4.0".to_string())
        );
    }

    #[test]
    fn appends_version_number_to_thor_binary_path() {
        let context = Context::test();
        assert_eq!(
            get_thor_binary_path(&context, "0.4.0".to_string()),
            Some(context.cache_dir.join("thor@0.4.0").unwrap())
        );
    }
}
