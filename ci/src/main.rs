extern crate futures;
extern crate reqwest;
extern crate tokio_core;

use futures::{stream, Future, Stream};
use reqwest::async::Client;

const N: usize = 1;

fn main() {
    let client = Client::new();

    let urls = ["https://api.ipify.org", "https://api.ipify.org"];

    // let status = stream::iter_ok(urls.iter().cloned())
    //     .map(move |url| client.get(url).send().and_then(|res| Ok(res.status())))
    //     .buffer_unordered(N);
    let status = 
        .map(move |url| client.get(url).send().and_then(|res| Ok(res.status())))

    let work = status
        .for_each(|s| {
            println!("Status: {}", s.is_success());
            Ok(())
        }).map_err(|e| println!("error = {:?}", e));

    tokio::run(work);
}
