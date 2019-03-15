use std::str;
extern crate regex;
use regex::Regex;
// use std::io::File;

enum TYPES {
    int_c,
    char_c,
    long_c,
    float_c,
    double_c,
    long_doulbe_c
}

enum SYNTAX {
    openCurlBracket,
    closed_curlBracket,
    open_round_bracket,
    close_round_bracket,
    semicolon,
    return_statement
}


struct Token<'b> {
    word: &'b str,
    value: &'b str
}

struct Lexer<'a,'b> {
    keywords: Vec<&'a str>,
    token_list: Vec<Token<'b>>
}

impl<'a, 'b> Lexer<'a, 'b> {
    fn new(keywords: Option<Vec<&'a str>>) -> Lexer<'a, 'b> {
        let _keywords = match keywords {
            Some(x) => x,
            None => vec![]
        };
        let tk = vec![];
        Lexer {
            keywords: _keywords,
            token_list: tk
        }
    }

    fn extract_token(&self, text: &'b str) -> Vec<Token<'b>> {
        let re = Regex::new(r"\s+").unwrap();
        let mut extracted_tks = vec![];
        for s in re.split(text) {
            if self.keywords.iter().any(|ref k| **k == s) {
                let tk = Token {word: s, value: "ok"};
                extracted_tks.push(tk);
            }
        };
        extracted_tks
    }

    fn register_token(&mut self, token: Token<'b>) {
       self.token_list.push(token);
    }
}

fn main() {
    let y = "int a  = 1 ;";
    let keywords = vec!["int", "long", "{", "}", "=", ";"];
    let mut lexer = Lexer::new(Some(keywords));
    let val = lexer.extract_token(&y);
    println!("this {}", val[1].word);
}
