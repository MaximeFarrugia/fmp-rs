# fmp-rs

This library is a Rust wrapper around the [Financial Modeling Prep API](https://site.financialmodelingprep.com/developer/docs).

### How to use

Let's take a look at how to use this library.

We'll use the [general search api endpoint](https://site.financialmodelingprep.com/developer/docs#general-search-company-search) for this example.

Start by defining a result struct with the fields you want to fetch from the API, like this:

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeneralSearchResult {
    symbol: Option<String>,
    name: Option<String>,
    currency: Option<String>,
    stock_exchange: Option<String>,
    exchange_short_name: Option<String>,
}
```

Next, create a client instance, using your API key:

```rust
use fmp_rs::fmp::FMP;

let client = FMP::new("API_KEY")?;
```

Next, we will create the query we want to perform (i.e.: let's search for AAPL):

```rust
use std::borrow::Cow;
use fmp_rs::api::company_search::general_search::GeneralSearchBuilder;

let endpoint = GeneralSearchBuilder::default()
    .query(Cow::Borrowed("AAPL"))
    .build()?;
```

Finally, let's execute the request:

```rust
use fmp_rs::api::query::Query;

let res: Vec<GeneralSearchResult> = endpoint.query(&client).await?;
println!("{res:#?}");
```

The complete example looks like this:

```rust
use std::borrow::Cow;

use fmp_rs::{api::{company_search::general_search::GeneralSearchBuilder, query::Query}, fmp::FMP};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeneralSearchResult {
    symbol: Option<String>,
    name: Option<String>,
    currency: Option<String>,
    stock_exchange: Option<String>,
    exchange_short_name: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = FMP::new("API_KEY")?;
    let endpoint = GeneralSearchBuilder::default()
        .query(Cow::Borrowed("AAPL"))
        .build()?;

    let res: Vec<GeneralSearchResult> = endpoint.query(&client).await?;
    println!("{res:#?}");
    return Ok(());
}
```
