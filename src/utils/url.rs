use std::time::Duration;

use log::trace;

pub async fn validate_http_url(url: &String) -> bool {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();

    trace!("URL: Validating {0}...", url);
    client.get(url).send().await.is_ok()
}