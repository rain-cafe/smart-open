use std::{error::Error, path::Path, process::exit};
use log::{error, trace};

mod utils;
mod parsers;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let mut args = std::env::args();

    let name = args.next().unwrap();
    let uri = args.next().unwrap_or(String::from("."));

    let config = utils::config::read();
    
    if uri == "--help" {
        let name = Path::new(&name).file_name().unwrap().to_str().unwrap();

        println!("{name} - an intelligent and pluggable version of open");
        println!();
        println!("Usage:");
        println!();
        println!("{name}                       opens a browser at the git repo if one is present, otherwise opens the file explorer");
        println!("{name} https://rains.cafe    opens a browser at https://rains.cafe");
        println!("{name} rains.cafe            opens a browser at https://rains.cafe");
        println!("{name} /home                 opens the file explorer at /home");
        println!();
        println!("Configs:");
        println!();
        println!("Update the config located at {0}", utils::config::get_path());
    } else {
        let uri = parsers::parse(&uri, &config.parsers).await.expect("Unable to parse URI");

        if let Err(e) = open::that(&uri) {
            error!("Failed to open uri! {uri}");
            trace!("Error: {e}");
            exit(1);
        }
    }

    Ok(())
}
