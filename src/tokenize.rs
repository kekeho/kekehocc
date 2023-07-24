use std::collections::VecDeque;
use std::cmp::min;


#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Reserved,  // Symbol
    Num,  // Number
    Ident,  // Identifier
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub val: Option<i64>,
    pub string: String,
}



impl Token {
    pub fn new(kind: TokenKind, val: Option<i64>, s: String) -> Token {
        return Token {
            kind: kind,
            val: val,
            string: s,
        };
    }
}

struct LoadError;


pub fn tokenize(s: &String, symbols: &Vec<String>, ignore: &Vec<String>) -> VecDeque<Token> {
    let mut codes: VecDeque<char> = s.chars().collect();
    let mut tokens: VecDeque<Token> = VecDeque::new();

    loop {
        skip_ignore(&mut codes, ignore);

        if let Ok(t) = load_symbol(&mut codes, symbols) {
            tokens.push_back(
                Token::new(TokenKind::Reserved, None, t)
            );
            continue;
        }

        if let Ok(n) = is_digit(&mut codes) {
            tokens.push_back(
                Token::new(TokenKind::Num, Some(n), "".to_string())
            );
            continue;
        }

        if let Ok(id) = is_ident(&mut codes) {
            tokens.push_back(
                Token::new(TokenKind::Ident, None, id)
            );
            continue;
        }

        if codes.len() == 0 {
            break;
        }

        println!("{:?}, {:?}", codes, tokens);
        panic!("Failed to tokenize!");
    }

    return tokens;
}


fn is_ident(chars: &mut VecDeque<char>) -> Result<String, LoadError> {
    if chars.len() == 0 {
        return Err(LoadError{});
    }

    let mut result: Result<String, LoadError> = Err(LoadError{});

    for i in 0..chars.len() {
        if chars[i].is_ascii_lowercase() {
            result = Ok(chars.range(0..i+1).collect());
            continue;
        }

        break;
    }

    match result {
        Ok(s) => {
            for _ in 0..s.len() {
                chars.pop_front();
            }

            return Ok(s);
        }

        _ => {
            return Err(LoadError{});
        }
    }
}


fn is_digit(chars: &mut VecDeque<char>) -> Result<i64, std::num::ParseIntError> {
    if chars.len() == 0 {
        return "".parse();
    }

    let mut i: usize = 0;
    let s: String = chars.range(0..i+1).collect();
    let mut prev_result: Result<i64, std::num::ParseIntError> = s.parse();

    while i+1 <= chars.len() {
        let s: String = chars.range(0..i+1).collect();
        let r: Result<i64, std::num::ParseIntError> = s.parse();

        match r {
            Err(_) => {
                for _ in 0..i {
                    chars.pop_front();
                }
                return prev_result;
            }
            Ok(_) => {
                prev_result = r;
                i += 1;
            }
        }
    }

    for _ in 0..i {
        chars.pop_front();
    }
    return prev_result;
}



fn load_symbol(chars: &mut VecDeque<char>, symbols: &Vec<String>) -> Result<String, LoadError> {
    // longest match
    const MAX_SYMBOL_LEN: usize = 2;
    let mut result: Result<String, LoadError> = Err(LoadError{});
    for i in 0..min(MAX_SYMBOL_LEN, chars.len()) {
        let check: String = chars.range(0..i+1).collect();
        if symbols.contains(&check) {
            result = Ok(check);
        }
    }

    match &result {
        Ok(x) => {
            for _ in 0..x.len() {
                chars.pop_front();
            }
        }

        _ => {}
    }

    return result;
}


fn skip_ignore(chars: &mut VecDeque<char>, ignore: &Vec<String>) {
    if chars.len() == 0 {
        return;
    }

    for i in 0..chars.len() {
        let check: String = chars.range(0..i+1).collect();
        if ignore.contains(&check) {
            for _ in 0..i+1 {
                chars.pop_front();
            }
            return;
        }

        break;
    }
}