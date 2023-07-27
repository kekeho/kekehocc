use std::env;
use std::rc::Rc;
use std::collections::VecDeque;

mod tokenize;
use tokenize::{Token, tokenize};

mod gen;
use gen::Node;

// Syntax
// program    = stmt*
// stmt       = expr ";" | "return" expr ";"
// expr       = assign
// assign     = equality ("=" assign)?
// equality   = relational ("==" relational | "!=" relational)*
// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
// add        = mul ("+" mul | "-" mul)*
// mul        = unary ("*" unary | "/" unary)*
// unary      = ("+" | "-")? primary
// primary    = num | ident | "(" expr ")"



fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 2 {
        panic!("Number of arguments invalid");
    }

    let ignore: Vec<String> = [" ", "\t"].iter().map(|x| x.to_string()).collect();
    let symbols: Vec<String> = [
        "+", "-", "*", "/", "(", ")",
        "==", "!=", "<", "<=", ">", ">=",
        "=", ";",
    ].iter().map(|x| x.to_string()).collect();

    let mut tokens: VecDeque<Token> = tokenize(&argv[1], &symbols, &ignore);
    let (nodes, local_vars_cnt): (Vec<Rc<Node>>, usize) = gen::program(&mut tokens); 

    // Header
    println!(".intel_syntax noprefix");
    println!(".globl main");

    println!("main:");

    // Prologue
    println!("\tpush rbp");
    println!("\tmov rbp, rsp");
    println!("\tsub rsp, {}", local_vars_cnt*8);

    for n in nodes {
        Node::gen(&n);
        println!("\tpop rax");
    }

    // Epilogue
    println!("\tmov rsp, rbp");
    println!("\tpop rbp");
    println!("\tret");
}
