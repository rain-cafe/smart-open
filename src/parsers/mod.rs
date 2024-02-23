use std::fs;

pub mod git;
pub mod file;
pub mod url;

pub trait Parser {
    async fn parse(options: &ParseOptions) -> Option<String>;
}

enum ParserName {
    Git,
    File,
    Url,
}

impl ParserName {
    fn from_str(value: &str) -> ParserName {
        match value {
            "git" => ParserName::Git,
            "file" => ParserName::File,
            "url" => ParserName::Url,
            _ => panic!("Unknown parser, '{value}'")
        }
    }
}

pub struct ParseOptions {
    pub uri: String,
    pub uri_as_path: Option<String>,
}

impl ParseOptions {
    fn from(uri: &String) -> ParseOptions {
        ParseOptions {
            uri: uri.clone(),
            uri_as_path: if let Ok(absolute) = fs::canonicalize(&uri) { Some(absolute.to_str().unwrap().to_owned()) } else { None },
        }
    }
}

pub async fn parse(uri: &String, parsers: &Vec<String>) -> Option<String> {
    let options = ParseOptions::from(uri);

    for parser in parsers.iter() {
        let name = ParserName::from_str(&parser);

        let result = match name {
            ParserName::File => file::FileParser::parse(&options).await,
            ParserName::Git => git::GitParser::parse(&options).await,
            ParserName::Url => url::UrlParser::parse(&options).await,
        };

        if let Some(uri) = result {
            return Some(uri);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use crate::parsers::ParseOptions;

    #[test]
    fn parse_options_from_should_create_options() {
        let options = ParseOptions::from(&String::from("."));
        assert_eq!(options.uri, ".");
        assert_eq!(options.uri_as_path, Some(fs::canonicalize(env::current_dir().unwrap().to_str().unwrap().to_string()).unwrap().to_str().unwrap().to_string()));
    }
}