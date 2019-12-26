use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::attribute::Attribute;

pub struct Database {
    db_path: String, // Where are DB is stored on disk
    hashtable: HashMap<String, HashMap<String, Attribute>>,
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
            db_path: String::from("/tmp/sdb/"),
            hashtable: HashMap::new(),
        }
    }
    pub fn write(&mut self, path: &str, value: &str) -> bool {
        let valuestable = self.hashtable.get_mut(&path.to_string());
        match valuestable {
            Some(valuestable) => {
                //let mut valuestable = self.hashtable.get_mut(&path.to_string()).unwrap();
                let attr = valuestable.get(&value.to_string());
                match attr {
                    Some(attr) => {
                        let iattr = valuestable.get_mut(&value.to_string()).unwrap();
                        iattr.incr();
                    },
                    None => {
                        let mut iattr = Attribute::new(&value);
                        iattr.incr();
                        valuestable.insert(value.to_string(), iattr);
                    },
                }
            },
            None => {
                let mut newvaluestable = HashMap::new();
                let mut iattr = Attribute::new(&value);
                iattr.incr();
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
    pub fn get_attr(&mut self, path: &str, value: &str) -> String {        
        let valuestable = self.hashtable.get(&path.to_string());
        match valuestable {
            Some(valuestable) => {
                let attr = valuestable.get(&value.to_string());
                match attr {
                    Some(attr) => {
                        if (attr.ttl > 0) {
                            println!("{:?}", attr);
                        }
                        let jattr = serde_json::to_string(&attr);
                        return jattr.unwrap();                        
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
        return String::from("");
    }
}
