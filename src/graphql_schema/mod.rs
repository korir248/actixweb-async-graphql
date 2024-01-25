use async_graphql::{MergedSubscription, SimpleObject};

mod messages;
pub mod users;

#[derive(SimpleObject, Default)]
pub struct Mutation {
    users: users::mutation::UsersMut,
    messages: messages::mutation::MessagesMut,
}

#[derive(SimpleObject, Default)]
pub struct Query {
    users: users::query::UsersQuery,
    messages: messages::query::MessagesQuery,
}

#[derive(Default, MergedSubscription)]
pub struct Subscription(
    pub users::subscription::get_new_user::GetNewUser,
    pub messages::subscription::get_new_message::GetNewMessage,
);
