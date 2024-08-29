use async_graphql::{Object, Result};
use diesel_async::RunQueryDsl;

use crate::{create_pool, models::Message};

#[derive(Default)]
pub struct GetAllMessages;

#[Object]
impl GetAllMessages {
    pub async fn get_all_messages(&self) -> Result<Vec<Message>> {
        use crate::schema::messages::dsl::messages;

        let mut conn = create_pool().get().await?;

        let res: Vec<Message> = messages.load(&mut conn).await?;

        Ok(res)
    }
}
