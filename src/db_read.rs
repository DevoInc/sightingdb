use crate::db::Database;

pub fn read(db: &mut Database, path: &str, value: &str) {
    println!("path:{}", path);
    println!("value:{}", value);
    let cnt = db.get_count(path, value);
    println!("{}", cnt);
}
