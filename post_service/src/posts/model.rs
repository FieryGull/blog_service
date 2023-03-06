use crate::{
    schema::posts as posts_table,
    common_lib::{
        db,
        error_handler::CustomError
    }
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = posts_table)]
pub struct Post {
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Posts {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
}

impl Posts {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = &mut db::connection()?;
        let posts = posts_table::table.load::<Posts>(conn)?;
        Ok(posts)
    }

    pub fn find(id: Uuid) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let post = posts_table::table.filter(posts_table::id.eq(id)).first(conn)?;
        Ok(post)
    }

    pub fn create(post: Post) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let post = Post::from(post);
        let post = diesel::insert_into(posts_table::table)
            .values(post)
            .get_result(conn)?;
        Ok(post)
    }

    pub fn update(id: Uuid, post: Post) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let post = diesel::update(posts_table::table)
            .filter(posts_table::id.eq(id))
            .set(post)
            .get_result(conn)?;
        Ok(post)
    }

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
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