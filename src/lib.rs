//! Rust SDK for [tonapi.io](https://tonapi.io)
//!
//! # Example
//!
//! ```
//! use tonapi_rs::{TonApiClient, TonApiConfig};
//!
//! TonApiClient::new(TonApiConfig::<reqwest::Client> {
//!     base_url: "",
//!     api_key: "",
//!     ..Default::default()
//! });
//! ```
//!
//! # Custom HTTP client
//!
//! By default, reqwest is used as http client.
//! If you want switch it to another http client, you can:
//!
//! ```
//! use serde::de::DeserializeOwned;
//! use tonapi_rs::{HttpClient, RequestParams, TONAPIResult, TonApiClient, TonApiConfig};
//!
//! #[derive(Default)]
//! pub struct CustomHttpClient {}
//!
//! #[async_trait::async_trait]
//! impl HttpClient for CustomHttpClient {
//!     async fn send_request<R: DeserializeOwned>(&self, params: RequestParams) -> TONAPIResult<R>
//!     {
//!         // Add your logic here
//!         todo!()
//!     }
//! }
//!
//! TonApiClient::new(TonApiConfig {
//!     base_url: "",
//!     api_key: "",
//!     http_client: Some(CustomHttpClient {}),
//!     ..Default::default()
//! });
//! ```
mod base;
mod error;
mod http_client;
mod rest;

pub use base::{TonApiClient, TonApiConfig};
pub use error::{TONAPIError, TONAPIResult};
pub use http_client::base::{HttpClient, Methods, RequestParams};
