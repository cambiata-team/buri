use clap::{Parser, Subcommand};
use vfs::{PhysicalFS, VfsPath};

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

fn main() {
    let cli = Cli::parse();

    let root: VfsPath = PhysicalFS::new(std::env::current_dir().unwrap()).into();
    let mut vio = virtual_io::Vio::new();

    let result = match &cli.command {
        Some(Commands::Init { name }) => init::do_init(&root, &mut vio, name),
        None => Ok(()),
    };
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
