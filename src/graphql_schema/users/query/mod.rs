use async_graphql::MergedObject;

pub mod get_all_users;

#[derive(MergedObject, Default)]
pub struct UsersQuery(pub get_all_users::GetAllUsers);
