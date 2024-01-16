use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use clap::{Parser, Subcommand};
use mongo::{actix_server::routes::get_data, mongodb::connect_mongo};
use std::error::Error;
use std::io;
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli: CLI = CLI::parse();
    let connect_to_database: Result<(), io::Error> = match &cli.command {
        Arguments::Mongodb { uri } => {
            let mongo_client: mongodb::Client = connect_mongo(&uri).await.unwrap();
            HttpServer::new(move || {
                App::new()
                    .app_data(Data::new(mongo_client.clone()))
                    .service(get_data)
            })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
        }
        Arguments::MySQL { uri } => {
            println!("MySQL");
            HttpServer::new(move || App::new())
                .bind(("", 8080))?
                .run()
                .await
        }
        Arguments::Postgres { uri } => {
            println!("Postgres");
            HttpServer::new(move || App::new())
                .bind(("", 8080))?
                .run()
                .await
        }
    };
    Ok(())
}
