
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref TABLE: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
}

pub fn symbol_table(token: String, token_type: String) {
    let mut table = TABLE.lock().unwrap();
    table.push((token, token_type));
}

pub fn display_table() {
    let table = TABLE.lock().unwrap();
    println!("Symbol Table");
    for (token, token_type) in table.iter() {
        println!(" Type :  {}      Value : {} ", token_type, token);
    }
}
