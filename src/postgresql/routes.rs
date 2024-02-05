use actix_web::web;

use super::postgres::{find_one, index};
pub fn postgres_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/postgres")
            .route("/", web::get().to(index))
            .route("/find_one/{table}", web::get().to(find_one)),
    );
}
