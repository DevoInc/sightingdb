// extern crate base64;
// use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};

use crate::db::Database;

pub fn write(db: &mut Database, path: &str, value: &str) -> bool {
    // let decoded_val = decode_config(&value, URL_SAFE_NO_PAD);
    // match decoded_val {
    //     Ok(b64_val) => {
            // let str_val = std::str::from_utf8(&b64_val).unwrap();
    println!("Writing path:[{}] value:[{}]", path, value);

    db.write(path, value);
    return true;
    //     },
    //     Err(..) => {
    //         println!("Write Error: Reading base64 input");
    //         return false;
    //     },
    // }
}
