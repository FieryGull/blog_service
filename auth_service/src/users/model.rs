use crate::db;
use crate::error_handler::CustomError;
use crate::schema::users as users_table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


// #[derive(Serialize, Deserialize, AsChangeset, Insertable)]
// #[diesel(table_name = users_table)]
// pub struct DbInsertUser {
//     pub name: String,
//     pub email: String,
//     pub password: String,
// }

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[diesel(table_name = users_table)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}


impl User {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = &mut db::connection()?;
        let users = users_table::table.load::<User>(conn)?;
        Ok(users)
    }

    pub fn find(id: Uuid) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let user = users_table::table.filter(users_table::id.eq(id)).first(conn)?;
        Ok(user)
    }

    pub fn create(user: User) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let user = diesel::insert_into(users_table::table)
            .values(user)
            .get_result(conn)?;
        Ok(user)
    }
}

impl From<RegisterUserSchema> for User {
    fn from(RegisterUserSchema { name, email, password }: RegisterUserSchema) -> Self {
        User {
            id: Uuid::new_v4(),
            name: name.into(),
            email: email.into(),
            password: password.into(),
        }
    }
}