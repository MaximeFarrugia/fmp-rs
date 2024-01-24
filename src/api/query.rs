use async_trait::async_trait;

use super::{client::Client, error::ApiError};

#[async_trait]
pub trait Query<T, C>
where
    C: Client,
{
    async fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}
