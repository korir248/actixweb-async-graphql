use crate::schema::{messages, users};
use async_graphql::SimpleObject;
use diesel::{deserialize::Queryable, AsChangeset, Insertable, Selectable};

#[derive(Debug, SimpleObject, Clone, Selectable, Queryable)]
#[diesel(table_name = users)]
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

#[derive(Debug, SimpleObject, Clone, Selectable, Queryable)]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: i32,
    pub sender: i32,
    pub receiver: i32,
    pub text: String,
}

#[derive(AsChangeset, SimpleObject, Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub sender: i32,
    pub receiver: i32,
    pub text: String,
}
