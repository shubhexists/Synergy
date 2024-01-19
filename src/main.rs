use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use clap::{Parser, Subcommand};
use futures::lock::Mutex;
use mongo::mongoose::{
    delete_many, delete_one, drop_collection, drop_database, find_many, find_one,
    get_all_databases, index, insert_many, insert_one, show_collections_in_a_database,
};
use mongodb::{options::ClientOptions, Client};
use postgresql::postgres;
use std::io;
use std::sync::Arc;
use tokio_postgres::NoTls;
mod mongo;
mod postgresql;

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
                App::new().app_data(Data::new(client.clone())).service(
                    web::scope("/mongodb")
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
                        .route(
                            "/insert_many/{database}/{collection}",
                            web::post().to(insert_many),
                        )
                        .route(
                            "/delete_one/{database}/{collection}",
                            web::delete().to(delete_one),
                        )
                        .route(
                            "/delete_many/{database}/{collection}",
                            web::delete().to(delete_many),
                        )
                        .route("/get_all_databases", web::get().to(get_all_databases))
                        .route(
                            "/get_collections/{database}",
                            web::get().to(show_collections_in_a_database),
                        )
                        .route("drop_database/{database}", web::delete().to(drop_database))
                        .route(
                            "/drop_collection/{database}/{collection}",
                            web::delete().to(drop_collection),
                        ),
                )
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
