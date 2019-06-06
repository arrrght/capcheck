#[macro_use]
extern crate log;

use prometheus::{IntCounter, Encoder, IntGauge, Opts, Registry, TextEncoder};
use reqwest::Client;
use std::io::Read;
use std::time::Instant;

const CAP_NAME: &str = "cap2";

fn main() {
    env_logger::init();
    let cap_url = &std::env::var("CAP_URL").unwrap_or("http://127.0.0.1:8080".to_owned());
    println!("CAP_URL/Checking {}", cap_url);

    let reg = Registry::new();
    let retry_cnt = IntCounter::with_opts(Opts::new("retry_counter", "Retry counter").const_label("name", CAP_NAME)).unwrap();
    let gauge = IntGauge::with_opts(Opts::new("access_time", "Access time to CapMonster").const_label("name", CAP_NAME)).unwrap();
    let error = IntGauge::with_opts(Opts::new("error", "some error").const_label("name", CAP_NAME)).unwrap();

    reg.register(Box::new(retry_cnt.clone())).unwrap();
    reg.register(Box::new(gauge.clone())).unwrap();
    reg.register(Box::new(error.clone())).unwrap();

    let client = Client::new().post(&format!("{}/in.php", cap_url));
    println!("client: {:?}", client);
    let file_part = reqwest::multipart::Part::bytes(&include_bytes!("generate.jpg")[..])
        .file_name("generate.jpg")
        .mime_str("image/jpeg")
        .unwrap();
    let form = reqwest::multipart::Form::new()
        .text("method", "post")
        .part("file", file_part);

    let time_now = Instant::now();
    let mut response = client.multipart(form).send().unwrap();
    let mut response_body = String::new();
    response.read_to_string(&mut response_body).unwrap();
    let ans: Vec<&str> = response_body.split("|").collect();
    let ans_int: usize = ans[1].to_string().parse().unwrap();
    debug!("ANS1:: {}", ans_int);

    let mut ret: Option<String> = None;
    while ret.is_none() {
        retry_cnt.inc();
        let check_api = format!("{}/res.php?action=get&id={}", cap_url, ans_int);
        let req_get = reqwest::get(&check_api).unwrap().text();
        ret = match req_get.unwrap().as_ref() {
            "CAPCHA_NOT_READY" => {
                std::thread::sleep(std::time::Duration::from_millis(100));
                None 
            },
            x if x.starts_with("OK") => Some(x.to_string()),
            x => panic!("F>U>C>K {:?}", x),
        };
    }
    let time_elapsed = time_now.elapsed().subsec_millis();
    gauge.set(time_elapsed as i64);
    debug!("ANS2:: {}, msec: {}", ret.unwrap(), time_elapsed);

    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = reg.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    println!("{}", String::from_utf8(buffer).unwrap());
}
