use std::{error::Error, process::exit};
use log::{error, trace};

mod utils;
mod parsers;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let mut args = std::env::args();

    let uri = args.nth(1).unwrap_or(String::from("."));

    let uri = parsers::parse(&uri, &vec![
        String::from("url"),
        String::from("git"), 
        String::from("file"), 
    ]).await.expect("Unable to parse URI");

    if let Err(e) = open::that(&uri) {
        error!("Failed to open uri! {uri}");
        trace!("Error: {e}");
        exit(1);
    }

    Ok(())
}
