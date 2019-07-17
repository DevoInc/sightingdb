use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::attribute::Attribute;

pub struct Database {
    db_path: String, // Where are DB is stored on disk
    hashtable: HashMap<String, Attribute>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            db_path: String::from("/tmp/sdb/"),
            hashtable: HashMap::new(),
        }
    }
    pub fn write(&mut self, path: &str, value: &str) -> bool {
        let mut attr = self.hashtable.get_mut(&path.to_string());
        match attr {
            Some(attr) => { println!("Increment"); attr.incr(); },
            None => {
                let mut iattr = Attribute::new(&value);
                iattr.incr();
                self.hashtable.insert(path.to_string(), iattr);
            },
        }
        return true;
    }
    pub fn get_count(&mut self, path: &str, value: &str) -> u128 {
        let count: u128;
        let mut attr = self.hashtable.get_mut(&path.to_string());
        match attr {
            Some(attr) => { return attr.count(); },
            None => {
                return 0;
            },            
        };
    }
    pub fn get_attr(&mut self, path: &str, value: &str) -> String {
        let attr = self.hashtable.get_mut(&path.to_string()).unwrap();
        let jattr = serde_json::to_string(&attr);//(Attribute{value: ans.value.to_string(), count: ans.count, first_seen: ans.first_seen, last_seen: ans.last_seen})

        // println!("jattr: {:?}", jattr);
        
        return jattr.unwrap();
    }
}
