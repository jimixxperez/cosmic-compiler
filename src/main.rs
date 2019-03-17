use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
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
    open_curly_bra,
    closed_curly_bra,
    open_round_bra,
    close_round_bra,
    semicolon,
    ampersand,
}
//
// TOKENS:
// - identifier: x, color, UP
// - keyword: if, while, return
// - separator: }, (, ;
// - operator: +, <, =
// - literal: true, 6.02, "music"
// - comment: /* some comment */
//

enum Token {
    identifier(String),
    key_if,
    key_while,
    sep_curly_open,
    sep_curly_close,
    sep_round_open,
    sep_round_close,
    sep_semicolon,
    op_plus,
    op_minus,
    bool(bool),
    float(f32),
    int(i32),
    string(String),
    comment(String)
}

struct NewRegex<'a> {
    regex_str: &'a str,
    regex: Regex,
}

impl<'a> Hash for NewRegex<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.regex_str.hash(state);
    }
}

impl<'a> PartialEq for NewRegex<'a> {
    fn eq(&self, other: &NewRegex) -> bool {
        self.regex_str == other.regex_str
    }
}

impl<'a> Eq for NewRegex<'a> {}

impl<'a> fmt::Display for NewRegex<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "re: {}", self.regex_str) 
    } 
}

impl<'a> fmt::Debug for  NewRegex<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "re: {}", self.regex_str) 
    }
}

impl<'a> NewRegex<'a> {
    fn new(re: &'a str) -> NewRegex {
        NewRegex {
            regex_str: re,
            regex: Regex::new(re).unwrap()
        } 
    }
}


struct Lexer {
    pos: usize,
    buffer: Vec<char>,
    size: usize
}

impl Iterator for Lexer  {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        self.token()
    }
}

impl Lexer {
    fn new(text: &str) -> Lexer {
        Lexer {
            pos: 0,
            buffer: text.chars().collect(),
            size: text.chars().count()
        } 
    }

    fn token(&mut self) -> Option<String>{
        self.skip_non_tokens();
        if (self.pos >= self.size) {
            return None;
        }
        let c = self.buffer[self.pos];
        if Lexer::isalpha(c) {
            self.process_identifier();
            return Some("identifier".to_owned());
        } else if Lexer::isdigit(c){
            self.process_number();
            return Some("digit".to_owned());
        } else {
            self.pos = self.pos + 1;
            return Some("don t know".to_owned());
        }
    }

    fn isalpha(c: char) -> bool{
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn isdigit(c: char) -> bool{
        (c >= '0' && c <= '9')
    }

    fn isalphanum(c: char) -> bool{
        Lexer::isalpha(c) || Lexer::isdigit(c)
    }

    fn process_number(&mut self) {
        let mut endpos = self.pos + 1;
        while(endpos < self.size && Lexer::isdigit(self.buffer[endpos])) {
            endpos = endpos + 1;
        }
        println!("this is a number {:?}", &self.buffer[self.pos..endpos]);
        self.pos = endpos;
    }

    fn process_identifier(&mut self) {
        let mut endpos = self.pos + 1;
        while(endpos < self.size && Lexer::isalphanum(self.buffer[endpos])) {
            endpos = endpos + 1;
        }
        println!("this is an identifier {:?}", &self.buffer[self.pos..endpos]);
        self.pos = endpos;
    }

    fn skip_non_tokens(&mut self) {
        while (self.pos < self.size)  {
            let c = self.buffer[self.pos];
            if (c == ' ' || c == '\t' || c == '\r') {
                println!("skipping position {}", self.pos);
                if c == '\t' {
                    println!("found tab");
                }
                self.pos = self.pos + 1;
            } else {
                break; 
            }
        }
    }
}

// struct Lexer<'a,'b> {
//     keywords: Vec<&'a str>,
//     token_list: Vec<Token<'b>>
// }

// impl<'a, 'b> Lexer<'a, 'b> {
//     fn new(keywords: Option<Vec<&'a str>>) -> Lexer<'a, 'b> {
//         let _keywords = match keywords {
//             Some(x) => x,
//             None => vec![]
//         };
//         let tk = vec![];
//         Lexer {
//             keywords: _keywords,
//             token_list: tk
//         }
//     }
// 
//     fn extract_token(&self, text: &'b str) -> Vec<Token<'b>> {
//         let re = Regex::new(r"\s+").unwrap();
//         let mut extracted_tks = vec![];
//         for s in re.split(text) {
//             if self.keywords.iter().any(|ref k| **k == s) {
//                 let tk = Token {word: s, value: None};
//                 extracted_tks.push(tk);
//             }
//         };
//         extracted_tks
//     }
// 
//     fn register_token(&mut self, token: Token<'b>) {
//        self.token_list.push(token);
//     }
// }
//

fn main() {
    let y = "\t if(a  == 1) {} ;";
    let z = " a b 120 > ok";
    let mut lexer = Lexer::new(&z);
    //lexer.skip_non_tokens();
    //println!("{}", Lexer::isalpha(lexer.buffer[1]));
    for k in lexer {
        println!("{} ", k);
    }
    // let mut RegexMapper = HashMap::new();
    // RegexMapper.insert("if", NewRegex::new(r"(?:\w)if\(?"));
    //RegexMapper.insert("else", NewRegex::new(r"(?<!\w)else(?!\w)"));
    // let k = RegexMapper.get("if").unwrap().regex.is_match(y);
    // println!("{}, {:?}", k, RegexMapper);
    // let keywords = vec!["int", "long", "{", "}", "=", ";"];
    // let mut lexer = Lexer::new(Some(keywords));
    // let val = lexer.extract_token(&y);
    // println!("this {}", val[1].word);
}
