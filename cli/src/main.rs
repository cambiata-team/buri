use std::{env, os::unix::process::CommandExt, process::Command};

use context::Context;
use files::workspace_file::WORKSPACE_FILE_NAME;
use thor::{get_configured_thor_version, get_thor_binary_path};
use version::is_valid_version;
use virtual_io::{Vio, VirtualIo};

mod context;
mod thor;

const MUST_INITIALIZE_MESSAGE: &str = "
You must be in a workspace to use Buri.
Use `buri init` to create a new workspace.

$   buri init

Use `buri --help` for more information.";

// What if the WORKSPACE.toml exists but not the .buri-version?
// Use the latest version of Thor and create the .buri-version.

#[derive(Debug, PartialEq)]
pub enum CliError {
    VfsError,
    MustInitialize,
    MustSpecifyACommand,
    InvalidThorVersion,
}

async fn main_impl(
    context: Context,
    vio: &mut impl VirtualIo,
) -> Result<Option<(String, Vec<String>)>, CliError> {
    let workspace_file = context
        .root
        .join(WORKSPACE_FILE_NAME)
        .map_err(|_| CliError::VfsError)?;
    if !workspace_file.exists().unwrap_or(false) || !workspace_file.is_file().unwrap_or(false) {
        vio.print(MUST_INITIALIZE_MESSAGE);
        return Err(CliError::MustInitialize);
    }

    let configured_thor_version = get_configured_thor_version(&context);
    let thor_version = match configured_thor_version {
        Some(version) => version,
        None => {
            // TODO: download from the internet
            vio.print("No Thor version configured. Using latest version.");
            "latest".to_string()
        }
    };

    // Should never happen, but check just in case.
    if !is_valid_version(&thor_version) {
        vio.print("Invalid Thor version.");
        return Err(CliError::InvalidThorVersion);
    }

    let thor_binary_path = get_thor_binary_path(&context, thor_version);
    if !thor_binary_path.exists().unwrap_or(false) || !thor_binary_path.is_file().unwrap_or(false) {
        // TODO: download from the internet
        // 1. Download
        // 2. Extract
        // 3. Make executable
    }

    Ok(Some((
        thor_binary_path.read_to_string().unwrap(),
        context.args,
    )))
}

#[tokio::main]
pub async fn main() {
    let mut raw_args = env::args();
    raw_args.next(); // Skip the executable name
    let args = raw_args.collect::<Vec<String>>();

    let mut vio = Vio::new();
    let context = Context::new(args);

    let result = main_impl(context, &mut vio).await;
    match result {
        Ok(Some((exec, args))) => {
            // Only works on Unix systems.
            // https://stackoverflow.com/a/53479765/11506995
            Command::new(exec).args(args).exec();
        }
        Ok(None) => {}
        Err(_) => std::process::exit(1),
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use virtual_io::VioFakeBuilder;

    #[tokio::test]
    async fn command_that_is_not_init_outside_workspace_errors() {
        let mut vio = VioFakeBuilder::new()
            .expect_stdout(MUST_INITIALIZE_MESSAGE)
            .build();
        let mut context = Context::test();
        context.args.push("build".to_string());
        let result = main_impl(context, &mut vio).await;
        assert_eq!(result, Err(CliError::MustInitialize));
        assert_eq!(vio.get_actual(), vio.get_expected());
    }
}
