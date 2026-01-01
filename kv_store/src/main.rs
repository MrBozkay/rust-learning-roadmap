use clap::{Parser, Subcommand};
use kv_store::KvStore;
use anyhow::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut store = KvStore::open("log.kv")?;

    match cli.command {
        Commands::Set { key, value } => {
            store.set(key, value)?;
            println!("OK");
        }
        Commands::Get { key } => {
            if let Some(value) = store.get(key)? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Commands::Rm { key } => {
            match store.remove(key) {
                Ok(_) => println!("OK"),
                Err(e) => println!("Error: {}", e),
            }
        }
    }

    Ok(())
}
