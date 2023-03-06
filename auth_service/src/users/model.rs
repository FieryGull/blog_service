use crate::db;
use crate::error_handler::CustomError;
use crate::schema::users as users_table;
use crate::users::basic_auth::hash_password;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[diesel(table_name = users_table)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct FilteredUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
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

    pub fn find_by_id(id: Uuid) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let user = users_table::table.filter(users_table::id.eq(id)).first(conn)?;
        Ok(user)
    }

    pub fn find_by_email(email: &String) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let user = users_table::table.filter(users_table::email.eq(email)).first(conn)?;
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
            password: hash_password(&password),
        }
    }
}

impl From<User> for FilteredUser {
    fn from(User { id, name, email, .. }: User) -> Self {
        FilteredUser {
            id,
            name,
            email,
        }
    }
}
