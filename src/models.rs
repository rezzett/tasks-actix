use super::schema::{categories, tasks};
use diesel::{prelude::*, Insertable, Queryable, SqliteConnection};

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

impl Category {
    #[allow(unused)]
    pub fn all(conn: &SqliteConnection) -> Result<Vec<Self>, diesel::result::Error> {
        categories::table.load::<Category>(&*conn)
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
