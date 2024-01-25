use async_graphql::MergedObject;

pub mod get_all_messages;

#[derive(MergedObject, Default)]
pub struct MessagesQuery(pub get_all_messages::GetAllMessages);
