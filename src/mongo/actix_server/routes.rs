use std::collections::HashMap;

use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpResponse,
};
use futures::StreamExt;
use mongodb::{
    bson::{doc, Document},
    Client, Cursor,
};

#[derive(Debug, serde::Serialize)]
struct ErrorRes {
    error: String,
}

#[get{"/api/{database}/{collection}"}]
pub async fn get_data(
    mongo_client: web::Data<Client>,
    path: web::Path<(String, String)>,
    query_params: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let (database, collection) = path.into_inner();
    if database.is_empty() {
        let error_message: ErrorRes = ErrorRes {
            error: "Database is empty".to_string(),
        };
        return HttpResponse::BadRequest().json(error_message);
    }
    if collection.is_empty() {
        let error_message: ErrorRes = ErrorRes {
            error: "Collection is empty".to_string(),
        };
        return HttpResponse::BadRequest().json(error_message);
    }
    let collection: mongodb::Collection<Option<Document>> =
        mongo_client.database(&database).collection(&collection);
    let mut query: Document = doc! {};
    for (key, value) in query_params.iter() {
        query.insert(key, value);
    }
    let cursor: Result<Cursor<Option<Document>>, mongodb::error::Error> =
        collection.find(query, None).await;
    match cursor {
        Ok(mut cursor) => {
            let mut documents: Vec<Document> = Vec::new();
            while let Ok(Some(result)) = cursor.next().await.unwrap() {
                documents.push(result);
            }
            return HttpResponse::Ok().json(documents);
        }
        Err(e) => {
            let error_message: ErrorRes = ErrorRes {
                error: e.to_string(),
            };
            return HttpResponse::InternalServerError().json(error_message);
        }
    }
}

// #[post("/api")]
// pub fn post_data(mongo_client: web::Data<Client>) -> HttpResponse {
//     todo!()
// }

// #[put("")]
// pub async fn put_data(mongo_client: Client) -> HttpResponse {
//     todo!()
// }

// #[delete("")]
// pub async fn delete_data(mongo_client: Client) -> HttpResponse {
//     todo!()
// }
