use crate::schema::users;
use async_graphql::SimpleObject;
use diesel::{deserialize::Queryable, AsChangeset, Insertable, Selectable};

#[derive(Debug, SimpleObject, Clone, Selectable, Queryable)]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub email: String,
    pub password: String,
}

#[derive(AsChangeset, SimpleObject, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: Option<String>,
    pub email: String,
    pub password: String,
}
