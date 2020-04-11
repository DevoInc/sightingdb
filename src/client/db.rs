//mod attribute;

use std::collections::HashMap;

use crate::attribute::Attribute;

pub struct Database {
    db: HashMap<String, Attribute>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            db: HashMap::new(),
        }
    }
    pub fn write(&mut self, path: &str, value: &str) -> bool {
        let attr = self.db.get_mut(&path.to_string());
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
    pub fn get_count(&mut self, path: &str, _value: &str) -> u128 {
        let attr = self.db.get_mut(&path.to_string());
        match attr {
            Some(attr) => { return attr.count(); },
            None => {
                return 0;
            },            
        };
    }
}
