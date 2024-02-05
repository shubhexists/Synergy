use clap::{Parser, Subcommand};
use commands::init::init;
use commands::start::start;
use std::io;
mod commands;
mod mongo;
mod postgresql;


#[derive(Parser)]
#[command(author, version, about , long_about=None)]
#[command(propagate_version = true)]

struct CLI {
    #[command(subcommand)]
    command: Arguments,
}

#[derive(Subcommand)]
enum Arguments {
    /// Creates a configuration file for the serveur
    Init,
    /// Starts the server with the given configuration file
    Start,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli: CLI = CLI::parse();
    let _ = match &cli.command {
        Arguments::Init => {
            let _ = init();
        }
        Arguments::Start => {
            let _ = start().await;
        }
    };
    Ok(())
}
