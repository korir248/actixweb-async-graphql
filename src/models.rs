use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct User {
    pub user_id: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: String,
    pub user_type: String,
    pub is_verified: bool,
    pub verification_code: Option<String>,
}

impl From<backend::models::User> for User {
    fn from(user: backend::models::User) -> Self {
        Self {
            user_id: user.user_id,
            username: user.username,
            password: user.password,
            email: user.email,
            user_type: user.user_type,
            is_verified: user.is_verified,
            verification_code: user.verification_code,
        }
    }
}
