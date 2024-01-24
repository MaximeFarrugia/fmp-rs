use std::{borrow::Cow, ops::Deref};

use derive_builder::Builder;

use crate::api::{endpoint::Endpoint, params::QueryParams};

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GeneralSearch<'a> {
    query: Cow<'a, str>,
    #[builder(default)]
    limit: Option<u64>,
    #[builder(default)]
    exchange: Option<Cow<'a, str>>,
}

impl<'a> Endpoint for GeneralSearch<'a> {
    fn endpoint(&self) -> Cow<'static, str> {
        return "search".into();
    }

    fn params(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push("query", self.query.deref())
            .push_opt("limit", self.limit.map(|x| Cow::Owned(x.to_string())))
            .push_opt("exchange", self.exchange.as_deref());
        return params;
    }
}
