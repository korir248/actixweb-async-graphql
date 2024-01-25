use async_graphql::MergedObject;

mod send_message;

#[derive(Default, MergedObject)]
pub struct MessagesMut(pub send_message::SendMessage);
