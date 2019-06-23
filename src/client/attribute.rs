pub struct Attribute {
    value: String,
    first_seen: u32,
    last_seen: u32,
    count: u128,
}

impl Attribute {
    pub fn new(value: &str) -> Attribute {
        Attribute {
            value: String::from(value),
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

