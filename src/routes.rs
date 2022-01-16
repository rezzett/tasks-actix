use crate::models::*;
use crate::AppData;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use tera::Context;

#[derive(Deserialize)]
struct CategoryForm {
    category: String,
}

#[derive(Deserialize)]
struct TaskForm {
    task: String,
    category_id: i32,
}

#[get("/")]
async fn index(data: web::Data<AppData>) -> impl Responder {
    let categories = Category::all(&data.db).unwrap(); // TODO unwrap
    let tasks = Task::all_with_category(&data.db).unwrap();
    let title = "Home";

    let mut ctx = Context::new();
    ctx.insert("title", title);
    ctx.insert("categories", &categories);
    ctx.insert("tasks", &tasks);

    let rendered = data
        .tera
        .render("index.html", &ctx)
        .expect("Template not found 'index.html'");
    HttpResponse::Ok().body(rendered)
}

#[get("/addtask")]
async fn add_task_get(data: web::Data<AppData>) -> impl Responder {
    let categories = Category::all(&data.db).unwrap(); // TODO remove unwrap
    let title = "Add Task";

    let mut ctx = Context::new();
    ctx.insert("title", title);
    ctx.insert("categories", &categories);

    let rendered = data
        .tera
        .render("add_task.html", &ctx)
        .expect("Template not found 'add_task.html'");
    HttpResponse::Ok().body(rendered)
}

#[post("/addtask")]
async fn add_task_post(
    form_data: web::Form<TaskForm>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let res = match Task::add_task(&form_data.task, form_data.category_id, &app_data.db) {
        Ok(s) => format!("{} tasks added", s),
        Err(e) => format!("Error: {}", e),
    };
    HttpResponse::Ok().body(format!("<h2> {}</h2>", res))
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
async fn add_category_post(
    form_data: web::Form<CategoryForm>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let res = match Category::add_category(&form_data.category, &app_data.db) {
        Ok(s) => format!("{} records added", s),
        Err(e) => format!("Error: {}", e),
    };
    HttpResponse::Ok().body(format!("<h2> {} </h2>", res)) // TODO redirect home or err
}

#[get("/category/{id}")]
async fn category(data: web::Data<AppData>, path: web::Path<i32>) -> impl Responder {
    let cat_id = path.into_inner();
    let tasks = Task::by_category(&data.db, cat_id).unwrap(); // TODO unwrap
    let categories = Category::all(&data.db).unwrap(); // TODO unwrap
    let cat_name = Category::by_id(&data.db, cat_id).unwrap(); // TODO unwrap;

    let mut ctx = Context::new();
    ctx.insert("tasks", &tasks);
    ctx.insert("title", &cat_name.name);
    ctx.insert("categories", &categories);

    let rendered = data
        .tera
        .render("category.html", &ctx)
        .expect("Template not found 'category.html'");
    HttpResponse::Ok().body(rendered)
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
