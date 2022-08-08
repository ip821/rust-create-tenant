use serde::Deserialize;
use serde::Serialize;

use crate::http_client::HttpClient;
use crate::response::Response;

#[derive(Serialize)]
struct LoginRequest {
    #[serde(rename = "tenantDomainName")]
    tenant_domain_name: String,
    #[serde(rename = "UserNameOrEMail")]
    username: String,
    #[serde(rename = "password")]
    password: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

impl HttpClient {
    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Response<LoginResponse> {
        let request = LoginRequest {
            tenant_domain_name: String::from("dbo"),
            username: String::from(username),
            password: String::from(password),
        };

        return self.call("/api/security/authentication/login", &request).await;
    }
}

