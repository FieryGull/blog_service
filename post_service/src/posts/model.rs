use crate::db;
use crate::error_handler::CustomError;
use crate::schema::posts as posts_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = posts_table)]
pub struct Post {
    pub user_id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Posts {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
}

impl Posts {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = &mut db::connection()?;
        let posts = posts_table::table.load::<Posts>(conn)?;
        Ok(posts)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let post = posts_table::table.filter(posts_table::id.eq(id)).first(conn)?;
        Ok(post)
    }

    pub fn create(post: Post) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let post = Post::from(post); // Maybe error
        let post = diesel::insert_into(posts_table::table)
            .values(post)
            .get_result(conn)?;
        Ok(post)
    }

    pub fn update(id: i32, post: Post) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let post = diesel::update(posts_table::table)
            .filter(posts_table::id.eq(id))
            .set(post)
            .get_result(conn)?;
        Ok(post)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn =&mut db::connection()?;
        let res = diesel::delete(posts_table::table.filter(posts_table::id.eq(id)))
            .execute(conn)?;
        Ok(res)
    }
}

impl Post {
    fn from(post: Post) -> Post {
        Post {
            user_id: post.user_id,
            title: post.title,
            body: post.body,
        }
    }
}