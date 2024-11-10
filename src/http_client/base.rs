use crate::error::TONAPIResult;
use serde::de::DeserializeOwned;

/// HTTP methods
#[derive(Debug)]
pub enum Methods {
    Get,
    Post,
}

impl Default for Methods {
    fn default() -> Self {
        Methods::Get
    }
}

/// Params that passes in every http request in `HttpClient` trait
#[derive(Default, Debug)]
pub struct RequestParams {
    pub method: Methods,
    pub url: String,
    pub api_key: String,
}

/// All http requests goes using this crate. By implementing it you can use custom http client for TON API
#[async_trait::async_trait]
pub trait HttpClient: Default {
    /// HTTP request handler
    async fn send_request<R: DeserializeOwned>(&self, params: RequestParams) -> TONAPIResult<R>;
}
