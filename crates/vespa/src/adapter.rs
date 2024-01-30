use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{document::VespaDocument, query::SearchQuery, response::VespaResponse};

/// A generic Adapter for the Vespa search endpoint
pub trait SearchAdapter {
    fn new(api_base: String) -> Self;

    fn query<Model: for<'a> Deserialize<'a>>(
        &self,
        query: &SearchQuery,
    ) -> impl std::future::Future<Output = Result<Option<VespaDocument<Model>>>>;
}

/// A generica Adapter for the Vespa document endpoint
pub trait DocumentAdapter {
    fn new(api_base: String) -> Self;

    fn get<Model: for<'a> Deserialize<'a>>(
        &self,
        api_base: &str,
    ) -> impl std::future::Future<Output = Result<Option<VespaResponse<Model>>>>;

    fn create<Model: Serialize + for<'a> Deserialize<'a>>(
        &self,
        api_base: &str,
        payload: &Model,
    ) -> impl std::future::Future<Output = Result<Option<VespaResponse<Model>>>>;

    fn update<Model: Serialize + for<'a> Deserialize<'a>>(
        &self,
        api_base: &str,
        payload: &Model,
    ) -> impl std::future::Future<Output = Result<Option<VespaResponse<Model>>>>;

    fn remove<Model: for<'a> Deserialize<'a>>(
        &self,
        api_base: &str,
    ) -> impl std::future::Future<Output = Result<Option<VespaResponse<Model>>>>;
}
