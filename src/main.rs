use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use clap::{Parser, Subcommand};
use mongo::mongoose::{find_many, find_one, index, insert_one};
use mongodb::{options::ClientOptions, Client};
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

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli: CLI = CLI::parse();
    let _ = match &cli.command {
        Arguments::Mongodb { uri } => {
            let client_options: ClientOptions = ClientOptions::parse(&uri).await.unwrap();
            let client: Client = Client::with_options(client_options).unwrap();
            HttpServer::new(move || {
                App::new()
                    .app_data(Data::new(client.clone()))
                    .route("/", web::get().to(index))
                    .route("/find_one/{database}/{collection}", web::get().to(find_one))
                    .route(
                        "/find_many/{database}/{collection}",
                        web::get().to(find_many),
                    )
                    .route(
                        "/insert_one/{database}/{collection}",
                        web::post().to(insert_one),
                    )
            })
            .bind(("127.0.0.1", 8080))
            .unwrap()
            .run()
            .await
        }
        Arguments::MySQL { uri } => {
            println!("MySQL {uri}");
            HttpServer::new(move || App::new())
                .bind(("", 8080))?
                .run()
                .await
        }
        Arguments::Postgres { uri } => {
            println!("Postgres {uri}");
            HttpServer::new(move || App::new())
                .bind(("", 8080))?
                .run()
                .await
        }
    };
    Ok(())
}
