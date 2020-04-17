extern crate rustyline;
// extern crate dirs;
extern crate regex;
//extern crate clap;

use regex::Regex;
use rustyline::error::ReadlineError;
use rustyline::Editor;
//use clap::App;

fn main() {
    // App::new("Sighting DB")
    //     .version("0.0.1")
    //     .about("(C) 2019 Devo Inc.")
    //     .author("Written by Sebastien Tricaud")
    //     .get_matches();

    println!(
        "Sigthing DB - count attributes at scale\n(c) Devo Inc 2019 - Written by Sebastien Tricaud"
    );

    // let mut histfile = String::from("");
    // match dirs::home_dir() {
    //     Some(mut dir) => {
    //         dir = dir.join(".sightingdb_history");
    //         // histfile = dir.to_string_lossy().to_string();
    //     }
    //     None => {
    //         // histfile = String::from(".sightingdb_history");
    //     }
    // }

    let rw_re = Regex::new(r"^(r|w)\s(\S+)\s(.*)").unwrap();

    let mut rl = Editor::<()>::new();
    // rl.load_history(&histfile);
    loop {
        let readline = rl.readline("[sdb]> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let caps = rw_re.captures(&line).unwrap();

                let db_action = caps.get(1).map_or("", |m| m.as_str());
                let path = caps.get(2).map_or("", |m| m.as_str());
                let value = caps.get(3).map_or("", |m| m.as_str());

                if db_action == "r" {
                    println!("read namespace={}; value={}", path, value);
                }
                if db_action == "w" {
                    println!("write namespace={}; value={}", path, value);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-C");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    // rl.save_history(&histfile).unwrap();
}
// extern crate ansi_term;
// extern crate base64;

// use ansi_term::Colour::Red;
// use std::string::ToString;
// use base64::{encode,decode};

// fn main() {
//     let red_string = Red.paint("SightingDB Daemon").to_string();
//     println!("{}", encode(&red_string));

// }

// extern crate reqwest;

// use std::collections::HashMap;

// fn main() -> Result<(), Box<std::error::Error>> {
//     let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")?
//         .json()?;
//     println!("{:#?}", resp);
//     Ok(())
// }
