use std::collections::HashMap;

use actix_web::{web, Responder};

use crate::commands::init::AppState;

pub async fn index() -> impl Responder {
    "Hello from Postgres!"
}

pub async fn find_one(
    params: web::Path<String>,
    searches: web::Query<HashMap<String, String>>,
    client: web::Data<AppState>,
) -> impl Responder {
    let mut query: String = String::from("SELECT * FROM ");
    query.push_str(&params);
    query.push_str(" WHERE ");
    let mut count: usize = 0;
    for (key, value) in searches.iter() {
        if count > 0 {
            query.push_str(" AND ");
        }
        query.push_str(key);
        query.push_str(" = '");
        query.push_str(value);
        query.push_str("'");
        count += 1;
    }
    let database: &std::sync::Arc<futures::lock::Mutex<tokio_postgres::Client>> = &client.db;
    let client: futures::lock::MutexGuard<'_, tokio_postgres::Client> = database.lock().await;
    let rows: Vec<tokio_postgres::Row> = client.query(&query, &[]).await.unwrap();
    println!("{:?}", rows);
    println!("{query}");
    // let mut response: Vec<HashMap<String, String>> = Vec::new();
    // for row in rows {
    //     let mut map: HashMap<String, String> = HashMap::new();
    //     for (i, column) in row.columns().iter().enumerate() {
    //         map.insert(column.name().to_string(), row.get(i));
    //     }
    //     response.push(map);
    // }
    // web::Json(response)
    "Hello from Postgres!"
}
