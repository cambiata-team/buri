use std::{env, os::unix::process::CommandExt, process::Command};

use context::Context;
use files::workspace_file::WORKSPACE_FILE_NAME;
use virtual_io::{Vio, VirtualIo};

mod context;
mod thor;

const MUST_INITIALIZE_MESSAGE: &str = "
You must be in a workspace to use Buri.
Use `buri init` to create a new workspace.

$   buri init

Use `buri --help` for more information.";

const MUST_SPECIFY_A_COMMAND_MESSAGE: &str = "
Please specify a command.

Use `buri --help` for more information.

$   buri --help
";

// What if the WORKSPACE.toml exists but not the .buri-version?
// Use the latest version of Thor and create the .buri-version.

#[derive(Debug, PartialEq)]
pub enum CliError {
    VfsError,
    MustInitialize,
    MustSpecifyACommand,
}

//   1. Is there a workspace file?
//      - Find the current Thor version
//      - Check if that version of Thor exists
//          - If not, download and install it
//      - Call that version of Thor, passing through all arguments
//   2. Is there command init?
//      - Check CLI config file for latest installed version of Thor
//          - If config file does not exist, call version API for latest version of Thor and download. Create config file.
//      - Call that version of Thor, passing through all arguments
// x 3. Inform them to call `buri init` to initialize a workspace.
fn main_impl(
    context: Context,
    vio: &mut impl VirtualIo,
) -> Result<Option<(String, Vec<String>)>, CliError> {
    if context.args.is_empty() {
        vio.print(MUST_SPECIFY_A_COMMAND_MESSAGE);
        return Err(CliError::MustSpecifyACommand);
    }

    let workspace_file = context
        .root
        .join(WORKSPACE_FILE_NAME)
        .map_err(|_| CliError::VfsError)?;
    if workspace_file.exists().map_err(|_| CliError::VfsError)?
        && workspace_file.is_file().map_err(|_| CliError::VfsError)?
    {
        vio.println("Workspace already exists, no need to create a new one.");
        return Ok(None);
    }

    if let Some(first_arg) = context.args.get(0) {
        if first_arg == "init" {
            // Initialize repo
            return Ok(None);
        }
    }

    vio.print(MUST_INITIALIZE_MESSAGE);
    Err(CliError::MustInitialize)
}

pub fn main() {
    let mut raw_args = env::args();
    raw_args.next(); // Skip the executable name
    let args = raw_args.collect::<Vec<String>>();

    let mut vio = Vio::new();
    let context = Context::new(args);

    let result = main_impl(context, &mut vio);
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

    #[test]
    fn no_command_errors() {
        let mut vio = VioFakeBuilder::new()
            .expect_stdout(MUST_SPECIFY_A_COMMAND_MESSAGE)
            .build();
        let context = Context::test();
        let result = main_impl(context, &mut vio);
        assert_eq!(result, Err(CliError::MustSpecifyACommand));
        assert_eq!(vio.get_actual(), vio.get_expected());
    }

    #[test]
    fn must_specify_command_even_if_workspace_file_is_present() {
        let mut vio = VioFakeBuilder::new()
            .expect_stdout(MUST_SPECIFY_A_COMMAND_MESSAGE)
            .build();
        let context = Context::test();
        context
            .root
            .join(WORKSPACE_FILE_NAME)
            .unwrap()
            .create_file()
            .unwrap();
        let result = main_impl(context, &mut vio);
        assert_eq!(result, Err(CliError::MustSpecifyACommand));
        assert_eq!(vio.get_actual(), vio.get_expected());
    }

    #[test]
    fn command_that_is_not_init_outside_workspace_errors() {
        let mut vio = VioFakeBuilder::new()
            .expect_stdout(MUST_INITIALIZE_MESSAGE)
            .build();
        let mut context = Context::test();
        context.args.push("build".to_string());
        let result = main_impl(context, &mut vio);
        assert_eq!(result, Err(CliError::MustInitialize));
        assert_eq!(vio.get_actual(), vio.get_expected());
    }
}
