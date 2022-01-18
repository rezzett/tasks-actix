mod models;
mod routes;
mod schema;

use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use routes::*;
use std::env;
use tera::Tera;

#[macro_use]
extern crate diesel;

struct AppData {
    tera: Tera,
    db: SqliteConnection,
}

fn establish_conn() -> SqliteConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&db_url).expect("Connection failed")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let db = establish_conn();
        let app_data = AppData {
            tera: Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
                .expect("Failed to resolve the template directory."),
            db,
        };
        App::new()
            .data(app_data)
            .service(
                actix_files::Files::new(
                    "/static",
                    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("static"),
                )
                .show_files_listing(),
            )
            .service(index)
            .service(add_category_get)
            .service(add_category_post)
            .service(add_task_get)
            .service(add_task_post)
            .service(category)
            .service(delete_task)
            .service(error_page)
            .service(not_found)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
