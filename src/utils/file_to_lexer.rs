use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::minify;
use super::symbol_table;
#[derive(Debug, PartialEq)]
enum Token {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Assign,
    Comma,
    String,
    Number,
    NewLine,
    Boolean,
    EOF,
}
impl Lexer {
    fn consume(&mut self) {
        self.position += 1;
    }
    fn next_token(&mut self) -> Token {
        if let Some(c) = self.current_char() {
            match c {
                b'{' => self.braces(),
                b'}' => self.braces(),
                b'[' => self.brackets(),
                b']' => self.brackets(),
                b':' => self.assign(),
                b'\n' => self.new_line(),
                b't' | b'f' => self.is_boolean(),
                b'"' => self.is_string(),
                b',' => self.comma(),
                b'0'..=b'9' => self.number(),
                _ => self.end_of_file(),
            }
        } else {
            return self.end_of_file();
        }
    }
    fn end_of_file(&mut self) -> Token {
        return Token::EOF;
    }

    fn number(&mut self) -> Token {
        let start = self.position;
        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                self.consume();
            } else {
                break;
            }
        }

        let input_number = &self.input[start..self.position];
        symbol_table::symbol_table(input_number.to_string(), "Number".to_string());
        Token::Number
    }

    fn comma(&mut self) -> Token {
        if self.current_char().unwrap() == b',' {
            self.consume();
            symbol_table::symbol_table("Comma".to_string(), "Comma".to_string());
            return Token::Comma;
        } else {
            return self.end_of_file();
        }
    }
    fn new_line(&mut self) -> Token {
        while self.current_char().unwrap() != b'\n' {
            self.consume();
            return Token::NewLine;
        }
        return Token::EOF;
    }

    fn braces(&mut self) -> Token {
        if self.current_char().unwrap() == b'{' {
            self.consume();
            symbol_table::symbol_table("LBrace".to_string(), "LBrace".to_string());
            return Token::LBrace;
        } else if self.current_char().unwrap() == b'}' {
            self.consume();
            symbol_table::symbol_table("RBrace".to_string(), "RBrace".to_string());
            return Token::RBrace;
        } else {
            return self.end_of_file();
        }
    }

    fn brackets(&mut self) -> Token {
        if self.current_char().unwrap() == b'[' {
            self.consume();
            symbol_table::symbol_table("LBracket".to_string(), "LBracket".to_string());
            return Token::LBracket;
        } else if self.current_char().unwrap() == b']' {
            self.consume();
            symbol_table::symbol_table("RBracket".to_string(), "RBracket".to_string());
            return Token::RBracket;
        } else {
            return self.end_of_file();
        }
    }
    fn assign(&mut self) -> Token {
        if self.current_char().unwrap() == b':' {
            self.consume();
            symbol_table::symbol_table(":".to_string(), "Assign".to_string());
            return Token::Assign;
        } else {
            return self.end_of_file();
        }
    }
    fn is_string(&mut self) -> Token {
        let start = self.position;
        if self.current_char().unwrap() == b'"' {
            self.consume();
            while self.current_char().unwrap() != b'"' {
                self.consume();
            }
            self.consume();

            let input_string = &self.input[start..self.position];

            symbol_table::symbol_table(input_string.to_string(), "String".to_string());
            return Token::String;
        } else {
            return self.end_of_file();
        }
    }

    fn current_char(&self) -> Option<u8> {
        if self.position >= self.input.len() {
            None
        } else {
            Some(self.input.as_bytes()[self.position])
        }
    }
    fn is_boolean(&mut self) -> Token {
        let booleans = ["true", "false"];
        let start = self.position;
        while self.current_char().unwrap().is_ascii_alphabetic() {
            self.consume();
        }
        let input_boolean = &self.input[start..self.position];
        if booleans.contains(&input_boolean) {
            symbol_table::symbol_table(input_boolean.to_string(), "Boolean".to_string());
            return Token::Boolean;
        } else {
            return self.end_of_file();
        }
    }
}
struct Lexer {
    input: String,
    position: usize,
}

pub fn read_contents(path: &Path) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn create_lexer(input: String) {
    let mut lexer = Lexer {
        input: input.to_string(),
        position: 0,
    };

    loop {
        let token = lexer.next_token();
        if token == Token::EOF {
            symbol_table::display_table();
            break;
        }
    }
}

// main
pub fn file_to_lexer(input: &Path) {
    let contents = read_contents(input);
    let minified_contents = minify::minify(&contents);
    println!("Lexer:");
    create_lexer(minified_contents);
}
