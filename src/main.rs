use std::env;
use std::rc::Rc;
use std::collections::VecDeque;


#[derive(PartialEq, Debug)]
enum TokenKind {
    Reserved,  // Symbol
    Num,  // Number
}


#[derive(PartialEq, Debug)]
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
}


#[derive(PartialEq, Debug)]
enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num {value: i64},
}

#[derive(PartialEq, Debug)]
struct Node {
    kind: NodeKind,
    lhs: Option<Rc<Node>>,
    rhs: Option<Rc<Node>>,
}


impl Node {
    pub fn gen(n: &Rc<Node>) {
        match n.kind {
            NodeKind::Num {value: v} => {
                println!("\tpush {}", v);
                return
            }

            _ => {}
        }

        Self::gen(&n.lhs.clone().unwrap());
        Self::gen(&n.rhs.clone().unwrap());
        
        println!("\tpop rdi");
        println!("\tpop rax");

        match n.kind {
            NodeKind::Add => {
                println!("\tadd rax, rdi");
            }

            NodeKind::Sub => {
                println!("\tsub rax, rdi");
            }

            NodeKind::Mul => {
                println!("\timul rax, rdi");
            }

            NodeKind::Div => {
                println!("\tcqo");
                println!("\tidiv rdi");
            }

            _ => {}
        }

        println!("\tpush rax");
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
const SYMBOLS: [char; 6] = ['+', '-', '*', '/', '(', ')'];


// Syntax
// expr    = mul ("+" mul | "-" mul)*
// mul     = primary ("*" primary | "/" primary)*
// primary = num | "(" expr ")"


fn consume(tokens: &mut VecDeque<Token>, s: &str) -> bool {
    if tokens.len() > 0 && tokens[0].string == s.to_string() {
        tokens.pop_front();
        return true;
    }

    return false;
}


fn expr(tokens: &mut VecDeque<Token>) -> Rc<Node> {
    let mut node: Rc<Node> = mul(tokens);
    loop {
        if consume(tokens, "+") {
            node = Rc::new(Node {
                kind: NodeKind::Add,
                lhs: Some(node),
                rhs: Some(mul(tokens)),
            })
        } else if consume(tokens, "-") {
            node = Rc::new(Node {
                kind: NodeKind::Sub,
                lhs: Some(node),
                rhs: Some(mul(tokens)),
            })
        } else {
            return node;
        }
    }
}


fn mul(tokens: &mut VecDeque<Token>) -> Rc<Node> {
    let mut node: Rc<Node> = primary(tokens);

    loop {
        loop {
            if consume(tokens, "*") {
                node = Rc::new(Node {
                    kind: NodeKind::Mul,
                    lhs: Some(node.clone()),
                    rhs: Some(primary(tokens)),
                })
            } else if consume(tokens, "/") {
                node = Rc::new(Node {
                    kind: NodeKind::Div,
                    lhs: Some(node.clone()),
                    rhs: Some(primary(tokens)),
                })
            } else {
                return node;
            }
        }
    }
}


fn primary(tokens: &mut VecDeque<Token>) -> Rc<Node> {
    if consume(tokens, "(") {
        let node: Rc<Node> = expr(tokens);
        consume(tokens, ")");
        return node;
    }

    return Rc::new(Node {
        kind: NodeKind::Num { value: tokens.pop_front().unwrap().val.unwrap() },
        lhs: None,
        rhs: None,
    })
}


fn tokenize(s: &String) -> VecDeque<Token> {
    let codes: Vec<char> = s.chars().collect();
    let mut i: usize = 0;

    let mut tokens: VecDeque<Token> = VecDeque::new();

    while i < codes.len() {
        if IGNORE.contains(&codes[i]) {
            i += 1;
        } else if SYMBOLS.contains(&codes[i]){
            tokens.push_back(
                Token::new(TokenKind::Reserved, None, codes[i].to_string())
            );
            i += 1;
        } else if let (Ok(digit), digit_i) = is_digit(&codes[i..]) {
            tokens.push_back(
                Token::new(TokenKind::Num, Some(digit), "".to_string())
            );
            i += digit_i;
        } else {
            panic!("Failed to tokenize!");
        }
    }

    return tokens;
}


fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 2 {
        panic!("Number of arguments invalid");
    }

    let mut tokens: VecDeque<Token> = tokenize(&argv[1]);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let node: Rc<Node> = expr(&mut tokens); 
    Node::gen(&node);

    println!("\tpop rax");
    println!("\tret");
}
