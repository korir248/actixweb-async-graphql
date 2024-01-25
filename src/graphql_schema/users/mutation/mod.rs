pub mod login;
pub mod signup;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct UsersMut(pub signup::Signup, pub login::Login);
