use std::{borrow::Cow, ops::Deref};

use assert_impl::assert_impl;
use fmp_rs::api::endpoint::Endpoint;
use fmp_rs_derive::FmpEndpoint;

#[derive(FmpEndpoint)]
pub struct GeneralSearch<'a> {
    query: Cow<'a, str>,
    limit: Option<u64>,
    exchange: Option<Cow<'a, str>>,
}

#[derive(FmpEndpoint)]
pub struct GeneralSearchWhereClause<'a, T>
where
    T: ToString,
{
    query: &'a T,
    limit: Option<u64>,
    exchange: Option<T>,
}

#[derive(FmpEndpoint)]
#[fmp(endpoint = "search")]
pub struct GeneralSearchAttributes<'a> {
    query: Cow<'a, str>,
    limit: Option<u64>,
    exchange: Option<Cow<'a, str>>,
}

#[test]
fn test_macro_exists() {
    let endpoint = GeneralSearch {
        query: Cow::Borrowed("AAPL"),
        limit: Some(10),
        exchange: None,
    };
}

#[test]
fn test_macro_implements_endpoint() {
    let endpoint = GeneralSearch {
        query: Cow::Borrowed("AAPL"),
        limit: Some(10),
        exchange: None,
    };
    assert_impl!(Endpoint: GeneralSearch);

    let endpoint = GeneralSearchWhereClause::<i32> {
        query: &42,
        limit: Some(10),
        exchange: None,
    };
    assert_impl!(Endpoint: GeneralSearch);
}

#[test]
fn test_macro_attributes() {
    let endpoint = GeneralSearchAttributes {
        query: Cow::Borrowed("AAPL"),
        limit: Some(10),
        exchange: None,
    };
    assert!(endpoint.endpoint().deref() == "search");
}
