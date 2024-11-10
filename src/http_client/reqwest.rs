use crate::error::TONAPIResult;
use crate::http_client::base::{HttpClient, RequestParams};
use crate::TONAPIError;
use serde::de::DeserializeOwned;

impl Into<reqwest::Method> for &crate::Methods {
    fn into(self) -> reqwest::Method {
        match self {
            crate::Methods::Get => reqwest::Method::GET,
            crate::Methods::Post => reqwest::Method::POST,
        }
    }
}

#[async_trait::async_trait]
impl HttpClient for reqwest::Client {
    async fn send_request<R: DeserializeOwned>(&self, params: RequestParams) -> TONAPIResult<R> {
        tracing::trace!("Preparing request");

        let url = reqwest::Url::parse(&params.url).map_err(|err| {
            tracing::error!("Cannot to parse url: {:?}", err);
            TONAPIError::PrepareRequestError(format!("Cannot to parse URL: {:?}", err))
        })?;
        let method = &params.method;
        let request = self
            .request(method.into(), url)
            .header("Authorization", format!("Bearer {}", params.api_key))
            .build()
            .map_err(|err| {
                tracing::error!("Cannot build request: {:?}", err);
                TONAPIError::PrepareRequestError(format!("Cannot build request : {:?}", err))
            })?;

        tracing::trace!("Sending request :{:?}", &params);

        let timer = std::time::Instant::now();

        let result = self.execute(request).await.map_err(|err| {
            tracing::error!("Cannot send request: {:?}", err);
            TONAPIError::SendRequestError(format!("Cannot send request: {:?}", err))
        })?;

        let time = timer.elapsed();

        tracing::trace!("Request time: {:?}", &time);

        if result.status().is_success() {
            result.json::<R>().await.map_err(|err| {
                tracing::error!("Cannot get response body: {:?}", err);
                TONAPIError::ParseRequestError(format!("Cannot get response body: {:?}", err))
            })
        } else {
            tracing::error!("Non success status code");
            Err(result.json::<TONAPIError>().await.map_err(|err| {
                tracing::error!("Cannot get response body: {:?}", err);
                TONAPIError::ParseRequestError(format!("Cannot get response body: {:?}", err))
            })?)
        }
    }
}
