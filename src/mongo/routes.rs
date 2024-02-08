use super::mongoose::{
    delete_many, delete_one, drop_collection, drop_database, find_many, find_one,
    get_all_databases, index, insert_many, insert_one, pop_first, pop_last, push_element,
    rename_field, show_collections_in_a_database, update_many, update_one,
};
use actix_web::web;

pub fn mongo_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/mongodb")
            .route("", web::get().to(index))
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
            )
            .route(
                "/update_one/{database}/{collection}",
                web::put().to(update_one),
            )
            .route(
                "/update_many/{database}/{collection}",
                web::put().to(update_many),
            )
            .route(
                "/rename_field/{database}/{collection}",
                web::put().to(rename_field),
            )
            .route(
                "/pop_last/{database}/{collection}/{field}",
                web::put().to(pop_last),
            )
            .route(
                "/pop_first/{database}/{collection}/{field}",
                web::put().to(pop_first),
            )
            .route(
                "/push_element/{database}/{collection}/{field}",
                web::post().to(push_element),
            ),
    );
}
