use crate::{
    common_lib::{db, error_handler::CustomError},
    schema::posts as posts_table,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = posts_table)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[diesel(table_name = posts_table)]
pub struct Posts {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub body: String,
}

impl Posts {
    pub fn find_all(user_id: Uuid) -> Result<Vec<Self>, CustomError> {
        let conn = &mut db::connection()?;
        let posts = posts_table::table
            .filter(posts_table::user_id.eq(user_id))
            .load::<Posts>(conn)?;
        Ok(posts)
    }

    pub fn find(id: Uuid, user_id: Uuid) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let post = posts_table::table
            .filter(posts_table::id.eq(id))
            .filter(posts_table::user_id.eq(user_id))
            .first(conn)?;
        Ok(post)
    }

    pub fn create(user_id: Uuid, post: NewPost) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let post = Posts::from(post, user_id);
        let post = diesel::insert_into(posts_table::table)
            .values(post)
            .get_result(conn)?;
        Ok(post)
    }

    pub fn update(id: Uuid, user_id: Uuid, post: NewPost) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let post = diesel::update(posts_table::table)
            .filter(posts_table::id.eq(id))
            .filter(posts_table::user_id.eq(user_id))
            .set(post)
            .get_result(conn)?;
        Ok(post)
    }

    pub fn delete(id: Uuid, user_id: Uuid) -> Result<usize, CustomError> {
        let conn = &mut db::connection()?;
        let res = diesel::delete(
            posts_table::table
                .filter(posts_table::id.eq(id))
                .filter(posts_table::user_id.eq(user_id)),
        )
        .execute(conn)?;
        Ok(res)
    }
}

impl Posts {
    fn from(NewPost { title, body }: NewPost, user_id: Uuid) -> Self {
        Posts {
            id: Uuid::new_v4(),
            user_id,
            title,
            body,
        }
    }
}
