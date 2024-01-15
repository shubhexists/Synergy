use std::{io, thread, time::Duration};

use mongo::mongodb::connect_mongo;
use ::mongodb::{
    bson::{doc, Document},
    Client, Collection,
};
use clap::{Parser, Subcommand};
// use mongodb::mongodb::connect_mongo;
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
    /// Enter your MongoDB URI to connect to the Database
    Mongodb {
        #[arg(short, long)]
        uri: String,
    },
    /// Enter your PostGres URI to connect to the PostGres Database
    Postgres {
        #[arg(short, long)]
        uri: String,
    },
    /// Enter your MySQL URI to connect to the MYSQL Database
    MySQL {
        #[arg(short, long)]
        uri: String,
    },
}

#[tokio::main]
async fn main() {
    let cli: CLI = CLI::parse();
    let connect_to_database = match &cli.command {
        Arguments::Mongodb { uri } => {
            connect_mongo(&uri).await;
        }
        Arguments::MySQL { uri } => {
            println!("MySQL");
        }
        Arguments::Postgres { uri } => {
            println!("Postgres");
        }
    };
}
