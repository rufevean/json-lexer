use std::path::Path;

mod utils;
use utils::file_to_lexer;
use utils::symbol_table;

fn main() {
    let path = Path::new("input_test/test2.json");
    file_to_lexer::file_to_lexer(path);
    symbol_table::add_table_into_a_file();

}


