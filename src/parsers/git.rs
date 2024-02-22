use log::trace;

use crate::utils::{command::is_program_not_in_path, git::get_remotes, url::validate_http_url};

use super::{Parser, ParseOptions};

pub struct GitParser {}

impl Parser for GitParser {
    async fn parse(options: &ParseOptions) -> Option<String> {
        if is_program_not_in_path("git") {
            return None;
        }

        if let Some(uri_as_path) = &options.uri_as_path {
            for remote in get_remotes(&uri_as_path) {
                if !validate_http_url(&remote.url).await {
                    continue;
                }
    
                return Some(remote.url);
            }
        }


        if let Some(path) = &options.uri_as_path {
            trace!("Path: {0}", path);
        }

        return None;
    }
}