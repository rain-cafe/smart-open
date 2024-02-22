use std::path::Path;

use super::{Parser, ParseOptions};

pub struct FileParser {}

impl Parser for FileParser {
    async fn parse(options: &ParseOptions) -> Option<String> {
        if let Some(path) = &options.uri_as_path {
            if Path::new(&path).exists() {
                return Some(format!("file://{0}", path.clone()));
            }
        }

        None
    }
}