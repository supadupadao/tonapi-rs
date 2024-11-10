use crate::{HttpClient, Methods, RequestParams, TONAPIResult, TonApiClient};
use serde::Deserialize;

static API_URL: &str = "/status";

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct StatusResult {
    pub rest_online: bool,
    pub indexing_latency: usize,
    pub last_known_masterchain_seqno: usize,
}

impl TonApiClient {
    /// Status
    pub async fn status(&self) -> TONAPIResult<StatusResult> {
        self.http_client
            .send_request(RequestParams {
                method: Methods::Get,
                url: format!("{}{}", self.base_url, API_URL),
                api_key: self.api_key.clone(),
            })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TonApiConfig;
    use httpmock::Method::GET;
    use httpmock::MockServer;

    #[tokio::test]
    async fn test_status() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/status")
                .header("Authorization", "Bearer Token");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    r#"{
              "rest_online": true,
              "indexing_latency": 9,
              "last_known_masterchain_seqno": 41873146
            }"#,
                );
        });

        let client = TonApiClient::new(TonApiConfig {
            base_url: &server.base_url(),
            api_key: "Token",
            ..Default::default()
        });

        let success_result = client.status().await.unwrap();
        mock.assert();
        assert_eq!(
            success_result,
            StatusResult {
                rest_online: true,
                indexing_latency: 9,
                last_known_masterchain_seqno: 41873146,
            }
        );
    }
}
