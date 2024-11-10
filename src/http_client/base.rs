/// HTTP methods
pub enum Methods {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Options,
    Custom(String),
}

/// Params that passes in every http request in `HttpClient` trait
pub struct RequestParams {
    pub method: Methods,
    pub url: String,
}

/// All http requests goes using this crate. By implementing it you can use custom http client for TON API
#[async_trait::async_trait]
pub trait HttpClient {
    /// HTTP request handler
    async fn execute(&self, params: RequestParams);
}
