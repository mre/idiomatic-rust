extern crate futures;
extern crate regex;
extern crate reqwest;
extern crate tokio_core;

#[macro_use]
extern crate failure;

mod error;

use futures::{future, Future};
use regex::Regex;
use reqwest::async::Client;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use error::CiError;

fn get_urls<T: Into<PathBuf>>(path: T) -> Result<Vec<String>, CiError> {
    let md_link = Regex::new(r"[^!]\[([^\[\]]+)\]\(([^#][^)]+)")?;

    let mut f = File::open(path.into())?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let mut urls = vec![];
    for cap in md_link.captures_iter(&contents) {
        urls.push(cap[2].to_string());
    }
    Ok(urls)
}

fn main() -> Result<(), CiError> {
    let file = env::args()
        .nth(1)
        .ok_or_else(|| CiError::Input("Missing input file".to_string()))?;

    let urls = get_urls(file)?;
    println!("Checking the following URLs:");
    urls.iter().for_each(|url| println!("{}", url));

    let client = Client::new();

    let mut requests = vec![];
    for url in &urls {
        let request = client
            .get(url)
            .send()
            .and_then(|res| res.error_for_status())
            .map_err(|e| println!("{}", e));
        requests.push(request);
    }

    let work = future::join_all(requests);

    let mut core = tokio_core::reactor::Core::new()?;
    if core.run(work).is_err() {
        std::process::exit(1);
    }
    Ok(())
}
