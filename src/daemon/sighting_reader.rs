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

    if path.starts_with("_config/") {
        let err = serde_json::to_string(&Message{message: String::from("No access to _config namespace from outside!")}).unwrap();
        return err;
    }
    
    let attr = db.get_attr(path, value);

    // Shadow Sightings
    let mut shadow_path: String = "_shadow/".to_owned();
    shadow_path.push_str(path);
    db.write(&shadow_path, value, 0);
    
    return attr;
}

// Our internal reading does not trigger shadow sightings
pub fn read_internal(db: &mut Database, path: &str, value: &str) -> String {
    return db.get_attr(path, value);
}
