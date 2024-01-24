use std::error::Error;

use bytes::Bytes;

use async_trait::async_trait;
use http::{request, response};
use url::Url;

use super::error::ApiError;

#[async_trait]
pub trait Client {
    type Error: Error + Send + Sync + 'static;

    fn url(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;

    async fn exec(
        &self,
        request: request::Builder,
        body: Vec<u8>,
    ) -> Result<response::Response<Bytes>, ApiError<Self::Error>>;
}
