

use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref TABLE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}


pub fn symbol_table(token: String, token_type: String) {
    let mut table = TABLE.lock().unwrap();
    table.insert(token, token_type);
}

pub fn display_table() {
    let table = TABLE.lock().unwrap();
    println!("Symbol Table");
    for (key, value) in table.iter() {
        println!("{}: {}", key, value);
    }
}



