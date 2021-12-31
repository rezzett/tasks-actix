use super::schema::{categories, tasks};
use diesel::{prelude::*, Insertable, Queryable, SqliteConnection};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "categories"]
pub struct NewCategory<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Associations, Identifiable, Serialize)]
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

impl Category {
    // TODO by id, delete
    pub fn all(conn: &SqliteConnection) -> Result<Vec<Self>, diesel::result::Error> {
        categories::table.load::<Category>(&*conn)
    }

    pub fn by_id(conn: &SqliteConnection, id: i32) -> Result<Self, diesel::result::Error> {
        categories::table.find(id).get_result::<Category>(&*conn)
    }

    pub fn add_category(
        name: &str,
        conn: &SqliteConnection,
    ) -> Result<usize, diesel::result::Error> {
        let new_category = NewCategory { name };
        diesel::insert_into(categories::table)
            .values(&new_category)
            .execute(&*conn)
    }
}

impl Task {
    // TODO task with cat name, by id, by category, delete
    pub fn all(conn: &SqliteConnection) -> Result<Vec<Self>, diesel::result::Error> {
        tasks::table.load::<Task>(&*conn)
    }

    pub fn all_with_category(
        conn: &SqliteConnection,
    ) -> Result<Vec<(Task, String)>, diesel::result::Error> {
        tasks::table
            .inner_join(categories::table.on(tasks::category_id.eq(categories::id)))
            .select((tasks::all_columns, categories::name))
            .load::<(Task, String)>(&*conn)
    }

    pub fn add_task(
        content: &str,
        category_id: i32,
        conn: &SqliteConnection,
    ) -> Result<usize, diesel::result::Error> {
        let new_task = NewTask {
            content,
            category_id,
        };
        diesel::insert_into(tasks::table)
            .values(&new_task)
            .execute(&*conn)
    }
}
