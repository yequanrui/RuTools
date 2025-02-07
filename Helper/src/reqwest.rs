use crate::{codec::decoder, common::create_file, i18n::get};
use reqwest::{header::USER_AGENT, Client, Response};
use serde_json::{Error, Value};
use std::{fs::File, io::Write};

pub const OPENX_PROJECT_ID: &str = "3960";

pub const OPENX_DOWNLOAD_PAGE: &str = "https://openx.huawei.com/WeTools/download";

pub const OPENX_TOKEN: &str = "NTEzZTBiMTEtZmUyOS00ZjMyLTk4Y2Yt";

pub const OPENX_DOWN_URL: &str = "https://openx.huawei.com/openxApi/project/release_id/download1";

pub async fn get_by_openx(url: String) -> Result<Response, Error> {
    let client = Client::builder().no_proxy().build().unwrap();
    let resp = client
        .get(url)
        .header("PRIVATE-TOKEN", decoder(OPENX_TOKEN))
        .send()
        .await
        .unwrap_or_else(|_| panic!("{}", get("request_failed")));
    Ok(resp)
}

pub async fn get_latest_package_ids(
    project_id: &str,
    package_name: &str,
) -> Result<Vec<String>, Error> {
    let url = format!(
        "https://openx.huawei.com/openapi/project/release/all/version1?projectId={project_id}"
    );
    let resp = get_by_openx(url).await?;
    let json: Value = resp.json::<Value>().await.unwrap();
    let data = json["data"].as_array().unwrap();
    let mut ids = Vec::new();
    let mut search_flag = false;
    for d in data {
        if search_flag {
            break;
        }
        for p in d["binaryPackages"]
            .as_array()
            .unwrap_or_else(|| panic!("Warning: {}", get("binary_not_exist")))
        {
            if p["tag"].to_string().contains(package_name) {
                ids.push(d["id"].to_string());
                ids.push(d["version"].to_string().replace("\"", ""));
                ids.push(p["id"].to_string());
                ids.push(p["tag"].to_string().replace("\"", ""));
                search_flag = true;
            }
        }
    }
    Ok(ids)
}

pub async fn down_latest_package(
    release_id: &str,
    package_id: &str,
    package_name: &str,
) -> Result<File, Error> {
    let url = format!(
        "https://openx.huawei.com/openapi/project/release/release_id/download1?releaseId={release_id}&packageId={package_id}"
    );
    let resp = get_by_openx(url).await?;
    // TODO 实现下载进度条显示
    let _total = resp.content_length().unwrap();
    let body = resp
        .bytes()
        .await
        .unwrap_or_else(|_| panic!("{}", get("response_failed")));
    let mut file = create_file(package_name);
    file.write_all(&body)
        .unwrap_or_else(|_| panic!("{}", get("write_failed")));
    Ok(file)
}

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
