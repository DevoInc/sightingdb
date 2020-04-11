use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use regex::Regex;

use crate::attribute::Attribute;

pub struct Database {
    db_path: String, // Where are DB is stored on disk
    hashtable: HashMap<String, HashMap<String, Attribute>>,
    re_stats: Regex,
}

#[derive(Serialize)]
pub struct DbError {
    error: String,
    path: String,
    value: String,
}

impl Database {
    pub fn new() -> Database {
        Database {
            db_path: String::from(""),
            hashtable: HashMap::new(),
            // "stats":{"1586548800":1},
            re_stats: Regex::new(r"\x22stats\x22:\{.+\},").unwrap(),
        }
    }
    pub fn set_db_path(&mut self, path: String) {
        self.db_path = path;
    }
    pub fn write(&mut self, path: &str, value: &str, timestamp: i64) -> bool {
        let valuestable = self.hashtable.get_mut(&path.to_string());
        match valuestable {
            Some(valuestable) => {
                //let mut valuestable = self.hashtable.get_mut(&path.to_string()).unwrap();
                let attr = valuestable.get(&value.to_string());
                match attr {
                    Some(attr) => {
                        let iattr = valuestable.get_mut(&value.to_string()).unwrap();
                        if timestamp > 0 {
                            iattr.incr_from_timestamp(timestamp);
                        } else {
                            iattr.incr();
                        }
                    },
                    None => {
                        let mut iattr = Attribute::new(&value);
                        if timestamp > 0 {
                            iattr.incr_from_timestamp(timestamp);
                        } else {
                            iattr.incr();
                        }
                        valuestable.insert(value.to_string(), iattr);
                    },
                }
            },
            None => {
                let mut newvaluestable = HashMap::new();
                let mut iattr = Attribute::new(&value);
                if timestamp > 0 {
                    iattr.incr_from_timestamp(timestamp);
                } else {
                    iattr.incr();
                }
                newvaluestable.insert(value.to_string(), iattr);
                self.hashtable.insert(path.to_string(), newvaluestable);
            },
        }
        return true;
    }
    pub fn get_count(&mut self, path: &str, value: &str) -> u128 {
        let count: u128;
        let valuestable = self.hashtable.get_mut(&path.to_string()).unwrap();
        let attr = valuestable.get_mut(&value.to_string());
        match attr {
            Some(attr) => { return attr.count(); },
            None => {
                return 0;
            },            
        };
    }
    pub fn get_attr(&mut self, path: &str, value: &str, with_stats: bool) -> String {        
        let valuestable = self.hashtable.get(&path.to_string());
        match valuestable {
            Some(valuestable) => {
                let attr = valuestable.get(&value.to_string());
                match attr {
                    Some(attr) => {
                        if (attr.ttl > 0) {
                            println!("{:?}", attr);
                        }

                        // FIXME: There MUST be a better way to handle the stats de-serialization
                        // in short I want to store stats with attributes, but at the same time
                        // not send them everytime one want to fetch an attribute, only
                        // when the user requests the statistics. Otherwise it can be rather large.
                        // I find regex more elegant (and faster) than deserializing to reserialize.
                        // Maybe I should use deserialize_with, but I could not find a great way to
                        // use it for what I want. Open to suggestions here :)
                        let jattr = serde_json::to_string(&attr).unwrap();
                        if with_stats {
                            return jattr;
                        }
                        let nostats = self.re_stats.replace(&jattr, "");
                        return nostats.to_string();                        
                    },
                    None => {
                        let err = serde_json::to_string(&DbError{error: String::from("Value not found"), path: path.to_string(), value: value.to_string()});
                        return err.unwrap();
                    }
                }
            },
            None => {
                let err = serde_json::to_string(&DbError{error: String::from("Path not found"), path: path.to_string(), value: value.to_string()});
                return err.unwrap();
            },
        }
        // return String::from(""); // unreachable statement, however I want to make it clear this is our default
    }
}
