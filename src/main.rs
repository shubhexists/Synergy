use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use clap::{Parser, Subcommand};
use futures::lock::Mutex;
use mongo::routes::mongo_config;
use mongodb::{options::ClientOptions, Client};
use postgresql::postgres;
use std::io;
use std::sync::Arc;
use tokio_postgres::NoTls;
mod mongo;
mod postgresql;

#[derive(Debug)]
pub struct AppState {
    pub db: Arc<Mutex<tokio_postgres::Client>>,
}

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
                    .configure(mongo_config)
            })
            .bind(("127.0.0.1", 8080))
            .unwrap()
            .run()
            .await
        }
        Arguments::Postgres { uri } => {
            let (client, connection) = tokio_postgres::connect(uri, NoTls)
                .await
                .expect("Failed to connect to Postgres.");
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("Connection error: {}", e);
                }
            });
            let client: Arc<Mutex<tokio_postgres::Client>> = Arc::new(Mutex::new(client));
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(AppState { db: client.clone() }))
                    .service(
                        web::scope("/postgres")
                            .route("/", web::get().to(postgres::index))
                            .route("/find_one/{table}", web::get().to(postgres::find_one)),
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
                .bind(("127.0.0.1", 8080))?
                .run()
                .await
        }
    };
    Ok(())
}
