use crate::http_client::base::{HttpClient, RequestParams};

#[async_trait::async_trait]
impl HttpClient for reqwest::Client {
    async fn execute(&self, _params: RequestParams) {
        todo!()
    }
}
