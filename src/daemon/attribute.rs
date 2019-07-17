use std::fmt;
use serde::Serialize;

#[derive(Serialize)]
pub struct Attribute {
    pub value: String,
    pub first_seen: u32,
    pub last_seen: u32,
    pub count: u128,
}

impl Attribute {
    pub fn new(value: &str) -> Attribute {
        Attribute {
            value: String::from(value), // FIXME: change to Vec<u8>
            first_seen: 0,
            last_seen: 0,            
            count: 0,
        }
    }
    pub fn count(&mut self) -> u128 {
        return self.count;
    }
    pub fn incr(&mut self) {
        self.count += 1;
    }
}

impl fmt::Debug for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute {{ value: {}, first_seen: {}, last_seen: {}, count: {}}}", self.value, self.first_seen, self.last_seen, self.count)
    }
}
