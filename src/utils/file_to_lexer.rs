use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::symbol_table;
#[derive(Debug, PartialEq)]
enum Token {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Identifier,
    String,
    Number,
    True,
    False,
    Null,
    NewLine,
    EOF,
}
impl Lexer {
    fn consume(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.input.as_bytes()[self.position] == b' ' {
            self.consume();
        }
    }
    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if let Some(c) = self.current_char() {
            match c {
                b'{' => self.braces(),
                b'}' => self.braces(),
                b'[' => self.brackets(),
                b']' => self.brackets(),
                b':' => self.assign(),
                b'\n' => self.new_line(),
                b'"' => self.is_string(),
                _ => self.end_of_file(),
            }
        } else {
            return self.end_of_file();
        }
    }
    fn end_of_file(&mut self) -> Token {
            symbol_table::display_table();
            return Token::EOF;
    }
    fn new_line(&mut self)->Token {
        while self.current_char().unwrap() != b'\n' {
            self.consume();
            return Token::NewLine;
        }
        return Token::EOF;
    }

    fn braces(&mut self) -> Token {
        if self.current_char().unwrap() == b'{' {
            self.consume();
            symbol_table::symbol_table("LBrace".to_string(),"LBrace".to_string()); 
            return Token::LBrace;
        } else if self.current_char().unwrap() == b'}' {

            self.consume();
            symbol_table::symbol_table("RBrace".to_string(),"RBrace".to_string());
            return Token::RBrace;
        } else {
            return self.end_of_file();
        }
    }

    fn brackets(&mut self) -> Token{
        if self.current_char().unwrap() == b'[' {
            self.consume();
            symbol_table::symbol_table("LBracket".to_string(),"LBracket".to_string());
            return Token::LBracket;
        } else if self.current_char().unwrap() == b']' {
            self.consume();
            symbol_table::symbol_table("RBracket".to_string(),"RBracket".to_string());
            return Token::RBracket;
        } else {
            return self.end_of_file();
        }
    }
    fn assign(&mut self) -> Token {
        if self.current_char().unwrap() == b':' {
            self.consume();
            symbol_table::symbol_table("Colon".to_string(),"Colon".to_string());
            return Token::Colon;

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
            
            symbol_table::symbol_table(input_string.to_string() ,"String".to_string());
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
    println!("Contents of file:");
    println!("{:?}", contents);
    println!("Lexer:");
    create_lexer(contents);
}
