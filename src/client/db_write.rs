use crate::db::Database;

pub fn write(db: &mut Database, path: &str, value: &str) {
    println!("path:{}", path);
    println!("value:{}", value);
    db.write(path, value);
}
