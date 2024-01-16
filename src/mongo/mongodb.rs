use std::error::Error;

use mongodb::{options::ClientOptions, Client};

pub async fn connect_mongo<'a>(uri: &'a str) -> Result<Client, mongodb::error::Error> {
    let connection_options: Result<ClientOptions, mongodb::error::Error> =
        ClientOptions::parse(&uri).await;
    match connection_options {
        Ok(connection_options) => {
            let client: Client = Client::with_options(connection_options).unwrap();
            Ok(client)
        }
        Err(e) => {
            panic!("{e}")
        }
    }
}
