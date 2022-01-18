use crate::models::*;
use crate::AppData;
use actix_web::{get, http::header, post, web, HttpResponse, Responder};
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
    let categories = Category::all(&data.db).unwrap_or(vec![]);
    let tasks = Task::all_with_category(&data.db).unwrap_or(vec![]);
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
    let categories = Category::all(&data.db).unwrap_or(vec![]);
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
    match Task::add_task(&form_data.task, form_data.category_id, &app_data.db) {
        Ok(_) => redirect_to("/"),
        Err(_) => redirect_to("/error/DB ERROR: Failed to add a new task"),
    }
}

#[get("/task/{id}")]
async fn delete_task(data: web::Data<AppData>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match Task::delete(&data.db, id) {
        Ok(_) => redirect_to("/"),
        Err(_) => redirect_to("/error/DB ERROR: Failed to delete the task"),
    }
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
    match Category::add_category(&form_data.category, &app_data.db) {
        Ok(_) => redirect_to("/"),
        Err(_) => redirect_to("/error/something went wrong"),
    }
}

#[get("/category/{id}")]
async fn category(data: web::Data<AppData>, path: web::Path<i32>) -> impl Responder {
    let cat_id = path.into_inner();
    let tasks = Task::by_category(&data.db, cat_id).unwrap_or(vec![]);
    let categories = Category::all(&data.db).unwrap_or(vec![]);
    let cat_name = match Category::by_id(&data.db, cat_id) {
        Ok(name) => name,
        Err(_) => {
            return HttpResponse::Found()
                .header(
                    header::LOCATION,
                    "/error/DB ERROR: Failed to fetch category name",
                )
                .finish()
        }
    };

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

#[get("/error/{mgs}")]
async fn error_page(data: web::Data<AppData>, msg: web::Path<String>) -> impl Responder {
    let mut ctx = Context::new();
    let title = "Error";
    ctx.insert("title", title);
    ctx.insert("error_msg", &msg.into_inner());
    let rendered = data
        .tera
        .render("error.html", &ctx)
        .expect("Template not found 'error.html'");
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

fn redirect_to(url: &str) -> impl Responder {
    HttpResponse::Found().header(header::LOCATION, url).finish()
}
