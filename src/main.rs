use hyper::Client;
use hyper::rt::{self, Future, Stream};
use std::io::{self, Write};

fn main() {
    let url = "http://127.0.0.1:8088".parse::<hyper::Uri>().unwrap();
    let client = Client::new();
    rt::run(
        client.get(url).and_then(|res| {
            println!("Resp: {}\nHeaders: {:#?}", res.status(), res.headers());
            res.into_body().for_each(|chunk| {
                io::stdout().write_all(&chunk).map_err(|e| panic!("{}", e))
            })
        }).map(|_| {
            println!("\n\nDone");
        }).map_err(|err| {
            println!("ERR: {}", err);
        })
    );
}
