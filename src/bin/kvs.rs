use clap::{Parser, Subcommand};
use kvs::KvStore;
use std::process;


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// gets the value for a given key
    Get { key: String },
    /// sets a key to a certain value
    Set { key: String, value: String },
    /// removes a key
    Rm { key: String },
}

fn main() {
    let cli = Args::parse();

    let mut kvstore = KvStore::new();

    match &cli.command {
        Some(Commands::Get { key }) => {
            kvstore.get(key.to_string());
        }
        Some(Commands::Set { key, value }) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        Some(Commands::Rm { key }) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        None => todo!(),
    }
}
