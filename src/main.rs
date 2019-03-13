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
    fn new(keywords: Vec<&'a str>) -> Lexer<'a, 'b> {
        let tk = vec![];
        Lexer {
            keywords: keywords,
            token_list: tk
        }
    }

    fn extract_token(&self, text: &'b str) -> Vec<Token<'b>>{
        let tk = Token {
            word: text,
            value: "1"
        };
        vec![tk]
    }

    fn register_token(&mut self, token: Token<'b>) {
       self.token_list.push(token);
    }
}

fn matchSubElements(el: &str) -> &str {
   let x = "test".to_owned();
   let y = &el;
    println!("{}, {}", y, el);
    y
}

fn main() {
    let y = "int a  = 1;";
    let keywords = vec!["int", "long", "{", "}", "="];
    let mut lexer = Lexer::new(keywords);
    let val = lexer.extract_token(&y);
    println!("{}", val[0].word);
    let t = Token{
        word:"t",
        value:"value"
    } ;
    lexer.register_token(t);
    let re = Regex::new(r"\s+").unwrap();
    let mut i = 0;
    for s in re.split(&y) {
        matchSubElements(&s);
        println!("{} {}",i, s) ;
        i = i + 1;
    }
}
