use crate::db::Database;
use crate::attribute::Attribute;
use base64::{decode_config, URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Message {
    message: String
}

#[derive(Deserialize)]
struct NotFound {
    error: String,
    path: String,
    value: String
}

//{"error":"Path not found","path":"security_intelligence","value":"MTAuNTIuNjAuNjk"}
//{"value":"MTAuNTIuNjAuNjk","first_seen":1582161107,"last_seen":1582161107,"count":1,"tags":"","ttl":0}

pub fn read(db: &mut Database, path: &str, value: &str) -> String {
    let attr = db.get_attr(path, value);

    let ret_attr = serde_json::from_str::<NotFound>(&attr);
    match ret_attr {
        Ok(_) => {
            // NotFound is found :)
            let mut shadow_notfound: String = "_shadow/notfound/".to_owned();
            shadow_notfound.push_str(path);
            db.write(&shadow_notfound, value, 0);
        },
        Err(_) => {
            let mut shadow_found: String = "_shadow/found/".to_owned();
            shadow_found.push_str(path);            
            db.write(&shadow_found, value, 0);
        },
    }
    
    return attr;
}
