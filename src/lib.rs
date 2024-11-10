//! Rust SDK for [tonapi.io](https://tonapi.io)
//!
//! # Example
//!
//! ```
//! use tonapi::{TonApiClient, TonApiConfig};
//!
//! TonApiClient::new(TonApiConfig {
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
//! use tonapi::{HttpClient, RequestParams, TonApiClient, TonApiConfig};
//!
//! pub struct CustomHttpClient {}
//!
//! impl HttpClient for CustomHttpClient {
//!     fn execute(&self, params: RequestParams) {
//!         // Add your logic here
//!         todo!()
//!     }
//! }
//!
//! TonApiClient::new(TonApiConfig {
//!     base_url: "",
//!     api_key: "",
//!     http_client: Some(HttpClient {}),
//!     ..Default::default()
//! });
//! ```
mod base;
mod http_client;
mod rest;

pub use base::{TonApiClient, TonApiConfig};
pub use http_client::base::{HttpClient, Methods, RequestParams};
