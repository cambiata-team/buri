use config::ensure_version_is_in_config;
use context::Context;
use errors::CliError;
use files::workspace_file::WORKSPACE_FILE_NAME;
use impure::download_thor;
use std::{env, os::unix::process::CommandExt, process::Command};
use thor::{get_configured_thor_version, get_thor_binary_path, is_thor_version_downloaded};
use virtual_io::{Vio, VirtualIo};

mod config;
mod context;
mod errors;
mod impure;
mod security;
mod thor;
mod version_api;

async fn main_impl(
    context: Context,
    vio: &mut impl VirtualIo,
) -> Result<(String, Vec<String>), CliError> {
    let workspace_file = context
        .root
        .join(WORKSPACE_FILE_NAME)
        .map_err(|e| CliError::VfsError(e.to_string()))?;
    // if first command is not init
    if let Some(command) = context.args.get(0) {
        if command != "init"
            && (!workspace_file.exists().unwrap_or(false)
                || !workspace_file.is_file().unwrap_or(false))
        {
            return Err(CliError::MustInitialize);
        }
    }

    let configured_thor_version = get_configured_thor_version(&context);
    let thor_version = match configured_thor_version {
        Some(version) => {
            if is_thor_version_downloaded(&context, &version) {
                version
            } else {
                download_thor(&context, vio, Some(version)).await?
            }
        }
        // Use the latest version of thor.
        None => download_thor(&context, vio, None).await?,
    };

    ensure_version_is_in_config(&context, &thor_version)?;

    let thor_binary_path = get_thor_binary_path(&context, &thor_version);

    Ok((thor_binary_path.as_str().to_string(), context.args))
}

#[tokio::main]
pub async fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    let mut raw_args = env::args();
    raw_args.next(); // Skip the executable name
    let args = raw_args.collect::<Vec<String>>();

    let mut vio = Vio::new();
    let context = Context::new(args);

    let result = main_impl(context, &mut vio).await;
    match result {
        Ok((exec, args)) => {
            // Only works on Unix systems.
            // https://stackoverflow.com/a/53479765/11506995
            Command::new(exec).args(args).exec();
        }
        Err(e) => {
            println!("{e}");
            std::process::exit(1)
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::impure::DETERMINING_LATEST_VERSION_MESSAGE;
    use files::cli_config::{CliConfig, CLI_CONFIG_FILE_NAME};
    use virtual_io::VioFakeBuilder;

    #[tokio::test]
    async fn command_that_is_not_init_outside_workspace_errors() {
        let mut vio = VioFakeBuilder::new().build();
        let mut context = Context::test();
        context.args.push("build".to_string());
        let result = main_impl(context, &mut vio).await;
        assert_eq!(result, Err(CliError::MustInitialize));
    }

    #[tokio::test]
    async fn downloads_latest_thor_version_if_not_configured() {
        let mut vio = VioFakeBuilder::new()
            .expect_stdout(DETERMINING_LATEST_VERSION_MESSAGE)
            .expect_stdout("Downloading version 0.1.0...\n")
            .build();
        let context = Context::test();
        context
            .root
            .join(WORKSPACE_FILE_NAME)
            .unwrap()
            .create_file()
            .unwrap();
        main_impl(context, &mut vio).await.unwrap();
        assert_eq!(vio.get_actual(), vio.get_expected());
    }

    #[tokio::test]
    async fn downloads_version_if_configured_but_is_not_downloaded() {
        let context = Context::test();
        context
            .root
            .join(WORKSPACE_FILE_NAME)
            .unwrap()
            .create_file()
            .unwrap();
        let config_file = context.root.join(CLI_CONFIG_FILE_NAME).unwrap();
        let mut config_contents = CliConfig::default();
        config_contents.set_version("0.1.0").unwrap();
        write!(config_file.create_file().unwrap(), "{}", config_contents).unwrap();
        let mut vio = VioFakeBuilder::new()
            .expect_stdout("Downloading version 0.1.0...\n")
            .build();

        main_impl(context, &mut vio).await.unwrap();

        assert_eq!(vio.get_actual(), vio.get_expected());
    }

    #[tokio::test]
    async fn init_command_downloads_and_runs_thor_even_outside_a_workspace() {
        let context = Context::test();
        let mut vio = VioFakeBuilder::new().build();

        let result = main_impl(context, &mut vio).await;
        assert_ne!(result, Err(CliError::MustInitialize));
    }
}
