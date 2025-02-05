use reqwest::header::USER_AGENT;
use reqwest::{Client, Response};
use serde_json::Error;

use crate::i18n::get;

pub const OPENX_PROJECT_ID: &str = "";

pub const OPENX_DOWNLOAD_PAGE: &str = "";

pub async fn get_by_github(url: String) -> Result<Response, Error> {
    let client = Client::builder().no_proxy().build().unwrap();
    let resp = client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .unwrap_or_else(|_| panic!("{}", get("request_failed")));
    Ok(resp)
}

pub async fn get_latest_package_ids(
    project_id: &str,
    package_name: &str,
) -> Result<Response, Error> {
    let client = Client::builder().no_proxy().build().unwrap();
    let resp = client
        .get(project_id.to_owned() + package_name)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .unwrap_or_else(|_| panic!("{}", get("request_failed")));
    Ok(resp)
}
