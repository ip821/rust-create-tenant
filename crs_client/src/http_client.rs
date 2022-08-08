use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::response::Response;

pub struct HttpClient {
    base_url: String,
    client: Client,
}

impl HttpClient {
    pub fn new(url: &String) -> HttpClient {
        HttpClient {
            base_url: url.clone(),
            client: Client::new(),
        }
    }

    pub async fn call<TRequest, TResponse>(
        &self,
        path: &str,
        request: &TRequest,
    ) -> Response<TResponse>
        where TRequest: Serialize, TResponse: DeserializeOwned
    {
        let response = self.client
            .post(self.base_url.clone() + path)
            .header(CONTENT_TYPE, "application/json")
            .json(&request)
            .send()
            .await
            .unwrap();

        return response
            .json::<Response<TResponse>>()
            .await
            .unwrap();
    }
}
