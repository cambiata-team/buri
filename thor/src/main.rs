use clap::{Parser, Subcommand};
use vfs::{PhysicalFS, VfsError, VfsPath};

mod init;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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
