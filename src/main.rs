extern crate reqwest;

use reqwest::Client;
use std::io::Read;
use std::time::Instant;

fn main() {
    let paste_api = "http://127.0.0.1:8088/in.php";
    let client = Client::new().post(paste_api);
    let form = reqwest::multipart::Form::new()
        .text("method", "post")
        .file("file", "generate.jpg")
        .unwrap();

    let time_now = Instant::now();
    let mut response = client.multipart(form).send().unwrap();
    let mut response_body = String::new();
    response.read_to_string(&mut response_body).unwrap();
    let ans: Vec<&str> = response_body.split("|").collect();
    let ans_int: usize = ans[1].to_string().parse().unwrap();
    println!("ANS1:: {}", ans_int);

    let mut ret: Option<String> = None;
    while ret.is_none() {
        let check_api = format!("http://127.0.0.1:8088/res.php?action=get&id={}", ans_int);
        ret = match reqwest::get(&check_api).unwrap().text().unwrap().as_ref() {
            "CAPCHA_NOT_READY" => None,
            x if x.starts_with("OK") => Some(x.to_string()),
            _ => panic!("F>U>C>K")
        };
    }
    let time_elapsed = time_now.elapsed().subsec_millis();
    println!("ANS2:: {}, msec: {}", ret.unwrap(), time_elapsed);
}
