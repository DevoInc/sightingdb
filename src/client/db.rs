//mod attribute;

use std::collections::HashMap;

use crate::attribute::Attribute;

pub struct Database {
    db_path: String, // Where are DB is stored on disk
    db: HashMap<String, Attribute>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            db_path: String::from("/tmp/sdb/"),
            db: HashMap::new(),
        }
    }
    pub fn write(&mut self, path: &str, value: &str) -> bool {
        let mut attr = self.db.get_mut(&path.to_string());
        match attr {
            Some(attr) => { attr.incr(); },
            None => {
                let mut iattr = Attribute::new(&value);
                iattr.incr();
                self.db.insert(path.to_string(), iattr);
            },
        }
        return true;
    }
    pub fn get_count(&mut self, path: &str, value: &str) -> u128 {
        let count: u128;
        let mut attr = self.db.get_mut(&path.to_string());
        match attr {
            Some(attr) => { return attr.count(); },
            None => {
                return 0;
            },            
        };
    }
}
