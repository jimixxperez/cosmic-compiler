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
    Identifier(String),
    Key_if,
    Key_while,
    Sep_curly_open,
    Sep_curly_close,
    Sep_round_open,
    Sep_round_close,
    Sep_semicolon,
    Op_plus,
    Op_minus,
    Bool(bool),
    Float(f32),
    Int(i32),
    String(String),
    Comment(String)
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Token::Identifier(x) => x,
            Token::String(x) => x,
            _ => "dunno"
        };
        write!(f, "{}", repr)
    }
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
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
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

    fn token(&mut self) -> Option<Token>{
        self.skip_non_tokens();
        if (self.pos >= self.size) {
            return None;
        }
        let c = self.buffer[self.pos];
        if Lexer::isalpha(c) {
            return Some(self.process_identifier().unwrap());
        } else if Lexer::isdigit(c){
            return Some(self.process_number().unwrap());
        }else if c == '"' {
            return Some(self.process_quote().unwrap());
        } else {
            self.pos = self.pos + 1;
            return None;
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

    fn process_quote(&mut self) -> Result<Token, String>{
        let mut endpos = self.pos + 1;
        while(endpos < self.size && self.buffer[endpos] != '"')  {
            endpos = endpos + 1;
        }
        if (endpos + 1 == self.size) {
            return Err("missing end quotes".to_owned());
        } else {
            let s: String = self.buffer[self.pos..endpos].iter().collect();
            if (endpos + 1 < self.size) {
                self.pos = endpos + 1;
            }
            return Ok(Token::String(s));
        }

    }

    fn process_number(&mut self) -> Result<Token, String>{
        let mut endpos = self.pos + 1;
        while(endpos < self.size && Lexer::isdigit(self.buffer[endpos])) {
            endpos = endpos + 1;
        }
        let s: String = self.buffer[self.pos..endpos].iter().collect();
        let num: i32  = s.parse().unwrap();
        self.pos = endpos;
        return Ok(Token::Int(num));
    }

    fn process_identifier(&mut self) -> Result<Token, String>{
        let mut endpos = self.pos + 1;
        while(endpos < self.size && Lexer::isalphanum(self.buffer[endpos])) {
            endpos = endpos + 1;
        }
        println!("this is an identifier {:?}", &self.buffer[self.pos..endpos]);
        let s: String = self.buffer[self.pos..endpos].iter().collect();
        self.pos = endpos;
        return Ok(Token::Identifier(s))
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
    let z = " a b 120 > ok \" ok \" ";
    let mut lexer = Lexer::new(&z);
    //lexer.skip_non_tokens();
    //println!("{}", Lexer::isalpha(lexer.buffer[1]));
    for k in lexer {
        println!("{} ", k);
    }
    println!("{}", z);
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
