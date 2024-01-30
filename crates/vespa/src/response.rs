use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    adapter::DocumentAdapter,
    fetch::{self, from_body},
};

fn format_document_uri(api_base: &str, path: &str) -> String {
    format!("{}/document/v1/{}", api_base, path)
}

pub struct VespaDocument {
    api_base: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VespaResponse<T> {
    pub path_id: String,
    pub id: String,
    pub fields: Option<T>,
}

impl DocumentAdapter for VespaDocument {
    /// Create a new Vespa adapter with the give base URL
    fn new(api_base: String) -> Self {
        Self { api_base }
    }

    async fn get<Model: for<'a> Deserialize<'a>>(
        &self,
        path: &str,
    ) -> Result<Option<VespaResponse<Model>>> {
        let uri = format_document_uri(&self.api_base, path);
        let res = fetch::get(&uri).await?;

        from_body(&res)
    }

    async fn create<Model: Serialize + for<'a> Deserialize<'a>>(
        &self,
        path: &str,
        payload: &Model,
    ) -> Result<Option<VespaResponse<Model>>> {
        let uri = format_document_uri(&self.api_base, path);
        let res = fetch::post(&uri, payload).await?;

        from_body(&res)
    }

    async fn update<Model: Serialize + for<'a> Deserialize<'a>>(
        &self,
        path: &str,
        payload: &Model,
    ) -> Result<Option<VespaResponse<Model>>> {
        let uri = format_document_uri(&self.api_base, path);
        let res = fetch::put(&uri, payload).await?;

        from_body(&res)
    }

    async fn remove<Model: for<'a> Deserialize<'a>>(
        &self,
        path: &str,
    ) -> Result<Option<VespaResponse<Model>>> {
        let uri = format_document_uri(&self.api_base, path);
        let res = fetch::delete(&uri).await?;

        from_body(&res)
    }
}
