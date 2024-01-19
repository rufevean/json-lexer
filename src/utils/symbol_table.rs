use std::fs::File;
use std::io::{self, Write};
use lazy_static::lazy_static;
use std::sync::Mutex;

struct Token{
   token_type: String,
   value: String,
}
impl Token{
    fn new(token_type: String, value :String) -> Token{
        Token{
            token_type: token_type.to_string(),
            value : value.to_string(),

        }
    }
}
lazy_static! {
    static ref TABLE: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
}

pub fn symbol_table(token: String, token_type: String) {
    let mut table = TABLE.lock().unwrap();
    table.push((token, token_type));
}

pub fn add_table_into_a_file() -> io::Result<()> {
    let table = TABLE.lock().unwrap();
    let mut output_table: Vec<Token> = Vec::new();
    for (token, token_type) in table.iter() {
        output_table.push(Token::new(token_type.to_string(), token.to_string()));
    }

    let mut file = File::create("output_symbol_table.json")?;

    for token in &output_table {
        writeln!(&mut file, "Type :  {}      Value : {}", token.token_type, token.value)?;
    }

    println!("Symbol table written to file");
    Ok(())
}
pub fn display_table() {
    let table = TABLE.lock().unwrap();
    println!("Symbol Table");
    for (token, token_type) in table.iter() {
        println!(" Type :  {}      Value : {} ", token_type, token);
    }
}
