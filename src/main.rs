use std::env;
use std::rc::Rc;
use std::collections::VecDeque;

mod tokenize;
use tokenize::{Token, tokenize};

mod gen;
use gen::Node;

// Syntax
// expr       = equality
// equality   = relational ("==" relational | "!=" relational)*
// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
// add        = mul ("+" mul | "-" mul)*
// mul        = unary ("*" unary | "/" unary)*
// unary      = ("+" | "-")? primary
// primary    = num | "(" expr ")"



fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 2 {
        panic!("Number of arguments invalid");
    }

    let ignore: Vec<String> = [" ", "\t"].iter().map(|x| x.to_string()).collect();
    let symbols: Vec<String> = [
        "+", "-", "*", "/", "(", ")",
        "==", "!=", "<", "<=", ">", ">=",
    ].iter().map(|x| x.to_string()).collect();

    let mut tokens: VecDeque<Token> = tokenize(&argv[1], &symbols, &ignore);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let node: Rc<Node> = gen::expr(&mut tokens); 
    Node::gen(&node);

    println!("\tpop rax");
    println!("\tret");
}
