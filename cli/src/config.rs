use crate::{context::Context, CliError};
use files::cli_config::{CliConfig, BURI_VERSION_KEY, CLI_CONFIG_FILE_NAME};
use toml_edit::{value, Document};

pub fn ensure_version_is_in_config(context: &Context, version: &str) -> Result<(), CliError> {
    let config_file = context.root.join(CLI_CONFIG_FILE_NAME).unwrap();
    if !config_file.exists().map_err(|_| CliError::InternalError)? {
        let mut config = CliConfig::default();
        config
            .set_version(version)
            .map_err(|_| CliError::InternalError)?;
        write!(
            config_file
                .create_file()
                .map_err(|_| CliError::InternalError)?,
            "{}",
            config
        )
        .map_err(|_| CliError::InternalError)?;
        return Ok(());
    }
    let contents = config_file
        .read_to_string()
        .map_err(|_| CliError::InternalError)?;
    let mut config = contents.parse::<Document>().unwrap();
    config[BURI_VERSION_KEY] = value(version);
    config_file
        .remove_file()
        .map_err(|_| CliError::InternalError)?;
    write!(
        config_file
            .create_file()
            .map_err(|_| CliError::InternalError)?,
        "{}",
        config
    )
    .map_err(|_| CliError::InternalError)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use files::cli_config::CliConfig;

    #[test]
    fn creates_config_file_if_it_does_not_exist() {
        let context = Context::test();
        let config_file = context.root.join(CLI_CONFIG_FILE_NAME).unwrap();
        assert!(!config_file.exists().unwrap());
        ensure_version_is_in_config(&context, "1.2.3").unwrap();
        assert!(config_file.exists().unwrap());
    }

    #[test]
    fn writes_version_to_file_if_not_exists() {
        let context = Context::test();
        let config_file = context.root.join(CLI_CONFIG_FILE_NAME).unwrap();
        ensure_version_is_in_config(&context, "1.2.3").unwrap();
        let contents = config_file.read_to_string().unwrap();
        let config = CliConfig::from(&contents).unwrap();
        assert_eq!(config.get_version(), Some("1.2.3".to_string()));
    }

    #[test]
    fn overwrites_version_in_file_if_exists() {
        let context = Context::test();
        let config_file = context.root.join(CLI_CONFIG_FILE_NAME).unwrap();
        ensure_version_is_in_config(&context, "1.2.3").unwrap();
        ensure_version_is_in_config(&context, "1.2.4").unwrap();
        let contents = config_file.read_to_string().unwrap();
        let config = CliConfig::from(&contents).unwrap();
        assert_eq!(config.get_version(), Some("1.2.4".to_string()));
    }

    #[test]
    fn edits_config_file() {
        let context = Context::test();
        let config_file = context.root.join(CLI_CONFIG_FILE_NAME).unwrap();
        config_file
            .create_file()
            .unwrap()
            .write_all(
                r#"
                some_random_key = true
                "#
                .as_bytes(),
            )
            .unwrap();
        ensure_version_is_in_config(&context, "1.2.4").unwrap();
        let contents = config_file.read_to_string().unwrap();
        assert!(contents.contains("some_random_key = true"));
    }
}
