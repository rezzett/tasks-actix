mod routes;

use actix_web::{App, HttpServer};
use routes::*;
use tera::Tera;

struct AppData {
    tera: Tera,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app_data = AppData {
            tera: Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap(),
        };
        App::new()
            .data(app_data)
            .service(actix_files::Files::new("/static", "static").show_files_listing())
            .service(index)
            .service(add_category_get)
            .service(add_category_post)
            .service(add_task_get)
            .service(add_task_post)
            .service(not_found)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
