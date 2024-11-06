use std::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum BodyError {
    #[error("failed to URL encode form parameters: {}", source)]
    UrlEncoded {
        #[from]
        source: serde_urlencoded::ser::Error,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError<E>
where
    E: Error,
{
    #[error("client error: {}", source)]
    Client { source: E },

    #[error("failed to parse JSON: {}", source)]
    Json {
        #[from]
        source: serde_json::Error,
    },

    #[error("failed to parse url: {}", source)]
    UrlParse {
        #[from]
        source: url::ParseError,
    },

    #[error("failed to create body: {}", source)]
    Body {
        #[from]
        source: BodyError,
    },
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    pub fn client(source: E) -> Self {
        return ApiError::Client { source };
    }
}
