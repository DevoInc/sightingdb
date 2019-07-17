extern crate base64;
use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};

use crate::db::Database;


pub fn write(db: &mut Database, path: &str, value: &str) {
    let decoded_val = decode_config(&value, URL_SAFE_NO_PAD).unwrap();
    let str_val = std::str::from_utf8(&decoded_val).unwrap();

    db.write(path, str_val);
}
