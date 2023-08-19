use clap::{Parser, Subcommand};
use vfs::{PhysicalFS, VfsError, VfsPath};

mod init;

#[derive(Parser)]
// bin_name = "buri" because the user will invoke the CLI by running `buri`,
// even though that's not the name of the actual binary.
#[command(author, version, about, long_about = None, bin_name = "buri")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Buri workspace
    Init {
        #[arg(long)]
        name: Option<String>,
    },
}

fn main() -> Result<(), VfsError> {
    let cli = Cli::parse();

    let root: VfsPath = PhysicalFS::new(std::env::current_dir().unwrap()).into();
    let mut vio = virtual_io::Vio::new();

    match &cli.command {
        Some(Commands::Init { name }) => init::do_init(&root, &mut vio, name),
        None => Ok(()),
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
