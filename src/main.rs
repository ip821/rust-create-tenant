use crs_client::http_client::HttpClient;

#[tokio::main]
async fn main() {
    let url = String::from("http://192.168.1.69:9000");
    let client = HttpClient::new(&url);
    let login_response = client.login("admin", "123").await;

    let token = &login_response.error_or_value.unwrap().token;

    println!("{}", token);
}
