use anyhow::Result;
use serde::Deserialize;

use crate::{
    adapter::SearchAdapter,
    document::VespaDocument,
    fetch::{self, from_body},
    query::SearchQuery,
};

pub struct VespaSearch {
    api_base: String,
}

/// The Vespa adapter, which impliments both the CRUD and
/// search methods
impl SearchAdapter for VespaSearch {
    /// Create a new Vespa adapter with the give base URL
    fn new(api_base: String) -> Self {
        Self { api_base }
    }

    /// Wraps the Vespa `query` API with a single parameter, the
    /// Vespa SearchQuery struct.
    async fn query<T: for<'a> Deserialize<'a>>(
        &self,
        search_query: &SearchQuery,
    ) -> Result<Option<VespaDocument<T>>> {
        let uri = format!("{}/search/", &self.api_base);
        let res = fetch::post(&uri, search_query).await?;

        from_body(&res)
    }
}
