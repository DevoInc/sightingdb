use crate::db::Database;
use crate::attribute::Attribute;
use base64::{decode_config, URL_SAFE_NO_PAD};

pub fn read(db: &mut Database, path: &str, value: &str) -> String {
    let decoded_val = decode_config(&value, URL_SAFE_NO_PAD).unwrap();
    let str_val = std::str::from_utf8(&decoded_val).unwrap();

    println!("Reading {}", str_val);
    
    return db.get_attr(path, str_val);
}
