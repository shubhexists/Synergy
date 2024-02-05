use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io,
    path::PathBuf,
    sync::Arc,
};

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use futures::lock::Mutex;
use mongodb::{options::ClientOptions, Client};
use tokio_postgres::NoTls;

use crate::{
    commands::config_text::get_config_file_text, mongo::routes::mongo_config, postgresql::postgres,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigLayout {
    pub database: String,
    pub uri: String,
    pub auth_header: Option<String>,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Database {
    Mongodb { uri: String },
    Postgres { uri: String },
    MySQL { uri: String },
}

#[derive(Debug)]
pub struct AppState {
    pub db: Arc<Mutex<tokio_postgres::Client>>,
}

impl Database {
    pub async fn run(&self) -> io::Result<()> {
        match self {
            Database::Mongodb { uri } => {
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
            Database::Postgres { uri } => {
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
            Database::MySQL { uri } => {
                println!("MySQL {uri}");
                HttpServer::new(move || App::new())
                    .bind(("127.0.0.1", 8080))?
                    .run()
                    .await
            }
        }
    }
}

pub fn init() -> io::Result<()> {
    let current_dir: PathBuf = std::env::current_dir().expect("Unable to get current directory.");
    let config_file: PathBuf = current_dir.join("config.yaml");
    File::create(&config_file).expect("Unable to create config file");
    println!("Config file created at: {:?}", config_file);
    let config_string: String = get_config_file_text();
    let _ = fs::write(&config_file, config_string).expect("Unable to write to config file.");
    Ok(())
}
