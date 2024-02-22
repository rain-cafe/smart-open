use std::time::Duration;

use log::trace;

use super::{Parser, ParseOptions};

pub struct UrlParser {}

impl Parser for UrlParser {
    async fn parse(options: &ParseOptions) -> Option<String> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(1))
            .build()
            .unwrap();

        let urls: [String; 2] = [
            format!("https://{0}", options.uri),
            format!("http://{0}", options.uri),
        ];

        for url in urls {
            trace!("URL: Validating {0}...", url);
            if client.get(&url).send().await.is_ok() {
                return Some(url);
            }
        }

        None
    }
}