use crate::http_client::base::HttpClient;

/// TON API client
pub struct TonApiClient<C: HttpClient = reqwest::Client> {
    _base_url: String,
    _api_key: String,
    _http_client: C,
}

/// Parameters of TON API client
#[derive(Default)]
pub struct TonApiConfig<'a, C: HttpClient = reqwest::Client> {
    /// Base url from which API paths will be built
    pub base_url: &'a str,
    /// TON API token from [tonconsole](https://tonconsole.com)
    pub api_key: &'a str,
    /// Use this parameter if you want change HTTP client. Reqwest is used by default
    pub http_client: Option<C>,
}

impl<C: HttpClient> TonApiClient<C> {
    /// Initialize new instance of TON API client
    pub fn new(config: TonApiConfig<C>) -> Self {
        Self {
            _base_url: config.base_url.to_owned(),
            _api_key: config.api_key.to_owned(),
            _http_client: config.http_client.unwrap_or_default(),
        }
    }
}
