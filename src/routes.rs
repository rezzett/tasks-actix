use crate::AppData;
use actix_web::{get, post, web, HttpResponse, Responder};
use tera::Context;

#[get("/")]
async fn index(data: web::Data<AppData>) -> impl Responder {
    let mut ctx = Context::new();
    let title = "Home";
    ctx.insert("title", title);
    let rendered = data
        .tera
        .render("index.html", &ctx)
        .expect("Template not found 'index.html'");
    HttpResponse::Ok().body(rendered)
}

#[get("/addtask")]
async fn add_task_get(data: web::Data<AppData>) -> impl Responder {
    let mut ctx = Context::new();
    let title = "Add Task";
    ctx.insert("title", title);
    let rendered = data
        .tera
        .render("add_task.html", &ctx)
        .expect("Template not found 'add_task.html'");
    HttpResponse::Ok().body(rendered)
}

#[post("/addtask")]
async fn add_task_post() -> impl Responder {
    HttpResponse::Ok().body("POST ADD TASK")
}

#[get("/addcategory")]
async fn add_category_get(data: web::Data<AppData>) -> impl Responder {
    let mut ctx = Context::new();
    let title = "Add Category";
    ctx.insert("title", title);
    let rendered = data
        .tera
        .render("add_category.html", &ctx)
        .expect("Template not found 'add_category.html'");
    HttpResponse::Ok().body(rendered)
}

#[post("/addcategory")]
async fn add_category_post() -> impl Responder {
    HttpResponse::Ok().body("POST ADD CATEGORY")
}

#[get("*")]
async fn not_found(data: web::Data<AppData>) -> impl Responder {
    let mut ctx = Context::new();
    let title = "Not Found";
    ctx.insert("title", title);
    let rendered = data
        .tera
        .render("404.html", &ctx)
        .expect("Temeplate not found '404.html'");
    HttpResponse::NotFound().body(rendered)
}
