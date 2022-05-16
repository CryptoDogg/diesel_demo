use crate::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub time: chrono::NaiveDateTime,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub time: &'a chrono::NaiveDateTime,
    pub title: &'a str,
    pub body: &'a str,
}
