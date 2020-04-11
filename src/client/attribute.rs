pub struct Attribute {
    value: String,
    count: u128,
}

impl Attribute {
    pub fn new(value: &str) -> Attribute {
        Attribute {
            value: String::from(value),
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

