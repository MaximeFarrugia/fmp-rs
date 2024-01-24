use async_trait::async_trait;
use bytes::Bytes;
use http::{request, response};
use url::Url;

use crate::api::{self, error::ApiError};

pub struct FMP {
    client: reqwest::Client,
    base_url: Url,
    api_key: String,
}

#[derive(Debug, thiserror::Error)]
pub enum FMPError {
    #[error("failed to parse url: {}", source)]
    UrlParse {
        #[from]
        source: url::ParseError,
    },

    #[error("api error: {}", source)]
    Api {
        #[from]
        source: ApiError<RestError>,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum RestError {
    #[error("reqwest error: {}", source)]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },

    #[error("http error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },
}

impl FMP {
    pub fn new(api_key: &str) -> Result<Self, FMPError> {
        let client = reqwest::Client::new();
        let base_url = Url::parse("https://financialmodelingprep.com/api/v3/")?;

        return Ok(Self {
            client,
            base_url,
            api_key: api_key.to_owned(),
        });
    }

    async fn auth_req(
        &self,
        request: request::Builder,
        body: Vec<u8>,
    ) -> Result<response::Response<Bytes>, ApiError<<Self as api::Client>::Error>> {
        use futures_util::TryFutureExt;
        let fct = || async {
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let resp = self.client.execute(request).await?;
            let mut http_resp = http::response::Response::builder()
                .status(resp.status())
                .version(resp.version());

            if let Some(http_headers) = http_resp.headers_mut() {
                for (key, value) in resp.headers() {
                    http_headers.insert(key, value.to_owned());
                }
            }
            return Ok(http_resp.body(resp.bytes().await?)?);
        };

        return fct().map_err(ApiError::client).await;
    }
}

#[async_trait]
impl api::Client for FMP {
    type Error = RestError;

    fn url(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        let url = self
            .base_url
            .join(endpoint)?
            .query_pairs_mut()
            .append_pair("apikey", self.api_key.as_str())
            .finish()
            .to_owned();

        return Ok(url);
    }

    async fn exec(
        &self,
        request: request::Builder,
        body: Vec<u8>,
    ) -> Result<response::Response<Bytes>, ApiError<<Self as api::Client>::Error>> {
        return self.auth_req(request, body).await;
    }
}
