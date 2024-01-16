use actix_web::{web, HttpResponse, Responder};
use bson::{doc, Document};
use futures::TryStreamExt;
use mongodb::{options::FindOptions, Cursor};
use std::collections::HashMap;

pub async fn index() -> impl Responder {
    "Hello From MongoDB!"
}

pub async fn find_one(
    params: web::Path<(String, String)>,
    searches: web::Query<HashMap<String, String>>,
    client: web::Data<mongodb::Client>,
) -> impl Responder {
    let db: mongodb::Database = client.database(&params.0);
    let collection: mongodb::Collection<Document> = db.collection::<Document>(&params.1);
    let mut query: Document = doc! {};
    for (key, value) in searches.iter() {
        query.insert(key, value);
    }
    match collection.find_one(query, None).await {
        Ok(result) => match result {
            Some(document) => HttpResponse::Ok().body(document.to_string()),
            None => HttpResponse::NotFound().body("No matching document found"),
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn find_many(
    params: web::Path<(String, String)>,
    searches: web::Query<HashMap<String, String>>,
    client: web::Data<mongodb::Client>,
) -> impl Responder {
    let db: mongodb::Database = client.database(&params.0);
    let collection: mongodb::Collection<Document> = db.collection::<Document>(&params.1);
    let mut query: Document = doc! {};
    for (key, value) in searches.iter() {
        query.insert(key, value);
    }
    let options: FindOptions = FindOptions::builder().build();
    let cursor: Result<Cursor<Document>, mongodb::error::Error> =
        collection.find(query, options).await;
    let mut response: Vec<Document> = Vec::new();
    match cursor {
        Ok(mut cursor) => {
            while let Ok(Some(doc)) = &cursor.try_next().await {
                response.push(doc.clone())
            }
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
