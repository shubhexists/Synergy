use std::error::Error;

use mongodb::{
    bson::{doc, Document},
    Client,
};

pub async fn connect_mongo<'a>(uri: &'a str) -> Result<Client, Box<dyn Error>> {
    let client: Result<Client, mongodb::error::Error> = Client::with_uri_str(&uri).await;
    match client {
        Ok(client) => Ok(client),
        Err(e) => {
            panic!("{e}");
        }
    }
}

// let db: mongodb::Database = client.database("ExpressTryDb");
//         let collection: mongodb::Collection<Document> = db.collection::<Document>("contacts");
//     let document = doc! {
//         "name" : "Shubham",
//         "email" : "fuckmylife@gmail.com",
//         "phone" : "99999999999"
//     };
//     let insert: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
//         collection.insert_one(document, None).await;
//     match insert {
//         Ok(insert) => {
//             println!("Document inserted successfully");
//         }
//         Err(e) => {
//             panic!("{e}")
//         }
//     }
//     Ok(())
