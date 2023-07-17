use std::env;


#[derive(PartialEq)]
enum TokenKind {
    Reserved,  // Symbol
    Num,  // Number
    EOF,
}


struct Token {
    kind: TokenKind,
    val: Option<i64>,
    string: String,
}


impl Token {
    pub fn new(kind: TokenKind, val: Option<i64>, s: String) -> Token {
        return Token {
            kind: kind,
            val: val,
            string: s,
        };
    }

    pub fn expect_number(&self) -> i64 {
        match self.kind {
            TokenKind::Num => {
                return self.val.unwrap();
            }

            _ => {
                panic!("Expected number");
            }
        }
    }
}


fn is_digit(chars: &[char]) -> (Result<i64, std::num::ParseIntError>, usize) {
    let mut i: usize = 0;
    let s: String = chars[0..i+1].into_iter().collect();
    let mut prev_result: Result<i64, std::num::ParseIntError> = s.parse();

    while i+1 <= chars.len() {
        let s: String = chars[0..i+1].into_iter().collect();
        let r: Result<i64, std::num::ParseIntError> = s.parse();

        match r {
            Err(_) => {
                return (prev_result, i);
            }
            Ok(_) => {
                prev_result = r;
                i += 1;
            }
        }
    }

    return (prev_result, i);
}


const IGNORE: [char; 2] = [' ', '\t',];


fn tokenize(s: &String) -> Vec<Token> {
    let codes: Vec<char> = s.chars().collect();
    let mut i: usize = 0;

    let mut tokens: Vec<Token> = vec![];


    while i < codes.len() {
        if IGNORE.contains(&codes[i]) {
            i += 1;
        } else if codes[i] == '+' || codes[i] == '-' {
            tokens.push(
                Token::new(TokenKind::Reserved, None, codes[i].to_string())
            );
            i += 1;
        } else if let (Ok(digit), digit_i) = is_digit(&codes[i..]) {
            tokens.push(
                Token::new(TokenKind::Num, Some(digit), codes[i].to_string())
            );
            i += digit_i;
        } else {
            panic!("Failed to tokenize!");
        }
    }

    tokens.push(Token::new(TokenKind::EOF, None, "".to_string()));
    return tokens;
}


fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 2 {
        panic!("Number of arguments invalid");
    }

    let tokens = tokenize(&argv[1]);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    println!("\tmov rax, {}", tokens[0].expect_number());

    let mut i :usize = 1;
    while i < tokens.len() {
        match tokens[i].kind {
            TokenKind::Reserved => {
                if tokens[i].string == "+".to_string() {
                    i += 1;
                    println!("\tadd rax, {}", tokens[i].expect_number());
                }
                if tokens[i].string == "-".to_string() {
                    i += 1;
                    println!("\tsub rax, {}", tokens[i].expect_number());
                }
            }   

            TokenKind::EOF => {
                println!("\tret");
                break;
            }

            _ => {
                panic!("Number token doubled");
            }
        }

        i += 1;
    }



}
