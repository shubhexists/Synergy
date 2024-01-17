use actix_web::{web, HttpResponse, Responder};
use bson::{doc, Document};
use futures::TryStreamExt;
use mongodb::{options::FindOptions, Cursor};
use serde_json::Value;
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
            //This should be in JSON
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

// THINK IF WE CAN IMPLEMENT A <T> for data param of the body..
pub async fn insert_one(
    params: web::Path<(String, String)>,
    body: web::Json<Value>,
    client: web::Data<mongodb::Client>,
) -> impl Responder {
    let db: mongodb::Database = client.database(&params.0);
    let collection: mongodb::Collection<Document> = db.collection::<Document>(&params.1);
    let data_value_to_be_inserted: &Value = body.get("value").unwrap_or_else(|| {
        panic!("The body should have a `value` tag with the data to be inserted in the collection")
    });
    let data_to_be_inserted: Document = bson::to_document(data_value_to_be_inserted)
        .map_err(|e| HttpResponse::InternalServerError().body(format!("Error: {}", e)))
        .unwrap();
    match collection.insert_one(data_to_be_inserted, None).await {
        Ok(result) => {
            let inserted_id: String = result
                .inserted_id
                .as_object_id()
                .unwrap()
                .to_hex()
                .to_string();
            HttpResponse::Ok().json(doc! {
                "inserted_id": inserted_id
            })
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn insert_many(
    params: web::Path<(String, String)>,
    client: web::Data<mongodb::Client>,
    body: web::Json<Value>,
) -> impl Responder {
    let db: mongodb::Database = client.database(&params.0);
    let collections: mongodb::Collection<Document> = db.collection::<Document>(&params.1);
    let values_to_be_inserted: Vec<Document> = body
        .get("values")
        .unwrap_or_else(|| {
            panic!("The body should have a `values` tag with the data to be inserted in the collection")
        })
        .as_array()
        .unwrap()
        .iter()
        .map(|value| bson::to_document(value).unwrap())
        .collect();

    match collections.insert_many(values_to_be_inserted, None).await {
        Ok(result) => {
            let inserted_ids: Vec<String> = result
                .inserted_ids
                .iter()
                .map(|(key, value)| {
                    format!(
                        "{}: {}",
                        key,
                        value.as_object_id().unwrap().to_hex().to_string()
                    )
                })
                .collect();
            HttpResponse::Ok().json(doc! {
                "inserted_ids": inserted_ids
            })
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn delete_one(
    params: web::Path<(String, String)>,
    client: web::Data<mongodb::Client>,
    searches: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let db: mongodb::Database = client.database(&params.0);
    let collection: mongodb::Collection<Document> = db.collection::<Document>(&params.1);
    let mut query: Document = doc! {};
    for (key, value) in searches.iter() {
        query.insert(key, value);
    }
    match collection.delete_one(query, None).await {
        Ok(result) => HttpResponse::Ok().json(doc! {
            "deleted_count": result.deleted_count.to_string()
        }),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn delete_many(
    params: web::Path<(String, String)>,
    client: web::Data<mongodb::Client>,
    searches: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let db: mongodb::Database = client.database(&params.0);
    let collection: mongodb::Collection<Document> = db.collection::<Document>(&params.1);
    let mut query: Document = doc! {};
    for (key, value) in searches.iter() {
        query.insert(key, value);
    }
    match collection.delete_many(query, None).await {
        Ok(result) => HttpResponse::Ok().json(doc! {
            "deleted_count": result.deleted_count.to_string()
        }),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn show_collections_in_a_database(
    params: web::Path<String>,
    client: web::Data<mongodb::Client>,
) -> impl Responder {
    let db: mongodb::Database = client.database(&params);
    let collections: Result<Vec<String>, mongodb::error::Error> =
        db.list_collection_names(None).await;
    match collections {
        Ok(collections) => HttpResponse::Ok().json(collections),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {e}")),
    }
}

pub async fn get_all_databases(client: web::Data<mongodb::Client>) -> impl Responder {
    let databases: Result<Vec<String>, mongodb::error::Error> =
        client.list_database_names(None, None).await;
    match databases {
        Ok(database) => HttpResponse::Ok().json(database),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
