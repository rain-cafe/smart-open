use std::time::Duration;

use log::trace;

pub async fn validate_http_url(url: &str) -> bool {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();

    trace!("URL: Validating {0}...", url);
    client.get(url).send().await.is_ok()
}

#[cfg(test)]
mod tests {
    use crate::utils::url::validate_http_url;

    #[tokio::test]
    async fn validate_http_url_should_return_true_for_valid_urls() {
        assert_eq!(validate_http_url("https://google.com").await, true);
    }

    #[tokio::test]
    async fn validate_http_url_should_return_false_for_invalid_urls() {
        assert_eq!(validate_http_url("https://./").await, false);
    }
}