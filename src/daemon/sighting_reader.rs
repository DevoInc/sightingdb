use crate::db::Database;
use crate::attribute::Attribute;
use base64::{decode_config, URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Message {
    message: String
}

pub fn read(db: &mut Database, path: &str, value: &str) -> String {
    let decoded_val = decode_config(&value, URL_SAFE_NO_PAD);
    match decoded_val {
        Ok(b64_val) => {
            let str_val = std::str::from_utf8(&b64_val).unwrap();
            return db.get_attr(path, str_val);
        },
        Err(..) => {
            println!("Read Error: Reading base64 input");
            let err = serde_json::to_string(&Message{message: String::from("Invalid base64 encoding (base64 url with non padding) value")});
            return err.unwrap();
        },
    }
    
}
