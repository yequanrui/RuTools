use crate::i18n::get;
use reqwest::header::USER_AGENT;
use reqwest::{Client, Response};
use serde_json::Error;

pub async fn get_by_github(url: String) -> Result<Response, Error> {
    let client = Client::builder().no_proxy().build().unwrap();
    let resp = client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .expect(get("request_failed"));
    Ok(resp)
}
