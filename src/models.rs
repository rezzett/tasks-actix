use super::schema::{categories, tasks};
use diesel::{Insertable, Queryable};

#[derive(Queryable)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "categories"]
pub struct NewCategory<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(Category)]
pub struct Task {
    pub id: i32,
    pub content: String,
    pub category_id: i32,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub content: &'a str,
    pub category_id: i32,
}
