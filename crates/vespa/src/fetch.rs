use anyhow::Result;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use spin_sdk::http::{send, Method, Response};

/// Converts a serializable struct into bytes, used for making
pub(crate) fn from_body<T: for<'a> Deserialize<'a>>(res: &Response) -> Result<T> {
    let doc: T = serde_json::from_slice(res.body())?;

    Ok(doc)
}

/// Converts a serializable struct into bytes, used for making
/// http requests
pub(crate) fn as_bytes<T: Serialize>(payload: &T) -> Result<Bytes> {
    Ok(serde_json::to_vec(payload)?.into())
}

/// A simple wrapper around the Spin `send_request` module
/// with a few application specific defaults.
/// The `application/json` content-type is hardwired into
/// this, which is required for Vespa requests
pub(crate) async fn fetch(
    uri: &str,
    method: Method,
    body: Option<Bytes>,
) -> Result<Response> {
    let res: Response = send(
        spin_sdk::http::Request::builder()
            .method(method)
            .header("Content-Type", "application/json")
            .uri(uri)
            .body(body)
            .build()
    )
    .await?;

    Ok(res)
}

/// A wrapper around a GET request using the `fetch` method
pub(crate) async fn get(uri: &str) -> Result<Response> {
    let res: Response = fetch(uri, Method::Get, None).await?;

    Ok(res)
}

/// A wrapper around a POST request using the `fetch` method
pub(crate) async fn post<T: Serialize>(uri: &str, payload: &T) -> Result<Response> {
    let res: Response = fetch(uri, Method::Post, Some(as_bytes(payload)?)).await?;

    Ok(res)
}

/// A wrapper around a PUT request using the `fetch` method
pub(crate) async fn put<T: Serialize>(uri: &str, payload: &T) -> Result<Response> {
    let res: Response = fetch(uri, Method::Put, Some(as_bytes(payload)?)).await?;

    Ok(res)
}

/// A wrapper around a DELETE request using the `fetch` method
pub(crate) async fn delete(uri: &str) -> Result<Response> {
    let res: Response = fetch(uri, Method::Delete, None).await?;

    Ok(res)
}
