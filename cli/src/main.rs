use std::env;

use files::workspace_file::WORKSPACE_FILE_NAME;
use vfs::{PhysicalFS, VfsPath};
use virtual_io::{Vio, VirtualIo};

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
fn main_impl(root: &VfsPath, vio: &mut impl VirtualIo, args: Vec<String>) -> Result<(), CliError> {
    let workspace_file = root
        .join(WORKSPACE_FILE_NAME)
        .map_err(|_| CliError::VfsError)?;
    if workspace_file.exists().map_err(|_| CliError::VfsError)? {
        vio.println("Workspace already exists, no need to create a new one.");
        return Ok(());
    }

    if args.is_empty() {
        vio.print(MUST_SPECIFY_A_COMMAND_MESSAGE);
        return Err(CliError::MustSpecifyACommand);
    }

    if let Some(first_arg) = args.get(0) {
        if first_arg == "init" {
            // Initialize repo
            return Ok(());
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
    let root: VfsPath = PhysicalFS::new(std::env::current_dir().unwrap()).into();

    let result = main_impl(&root, &mut vio, args);
    if result.is_err() {
        std::process::exit(1);
    }
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
        let root: VfsPath = PhysicalFS::new(std::env::current_dir().unwrap()).into();
        let result = main_impl(&root, &mut vio, vec![]);
        assert_eq!(result, Err(CliError::MustSpecifyACommand));
        assert_eq!(vio.get_actual(), vio.get_expected());
    }

    #[test]
    fn command_that_is_not_init_outside_workspace_errors() {
        let mut vio = VioFakeBuilder::new()
            .expect_stdout(MUST_INITIALIZE_MESSAGE)
            .build();
        let root: VfsPath = PhysicalFS::new(std::env::current_dir().unwrap()).into();
        let result = main_impl(&root, &mut vio, vec!["build".to_string()]);
        assert_eq!(result, Err(CliError::MustSpecifyACommand));
        assert_eq!(vio.get_actual(), vio.get_expected());
    }
}
