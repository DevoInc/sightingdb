use std::env;
use std::process;

extern crate base64;
use base64::{encode_config, URL_SAFE_NO_PAD};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Syntax: {} string", &args[0]);
        process::exit(1);
    }

    let strenc = &args[1];
    let encoded_val = encode_config(&strenc, URL_SAFE_NO_PAD);

    println!("{}", encoded_val);
}
