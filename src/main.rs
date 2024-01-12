use std::path::Path;

mod utils;
use utils::file_to_lexer;

fn main() {
    let path = Path::new("input_test/test.json");
    file_to_lexer::file_to_lexer(path);
}


