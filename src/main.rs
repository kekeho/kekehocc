use std::env;
use std::rc::Rc;
use std::cmp::min;
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
    Eq,
    Neq,
    Lt,
    Lte,
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

        // Add/Sub/Mul/Div/Cmp
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

            NodeKind::Eq => {
                println!("\tcmp rax, rdi");
                println!("\tsete al");
                println!("\tmovzb rax, al");
            }

            NodeKind::Neq => {
                println!("\tcmp rax, rdi");
                println!("\tsetne al");
                println!("\tmovzb rax, al");
            }

            NodeKind::Lt => {
                println!("\tcmp rax, rdi");
                println!("\tsetl al");
                println!("\tmovzb rax, al");
            }

            NodeKind::Lte => {
                println!("\tcmp rax, rdi");
                println!("\tsetle al");
                println!("\tmovzb rax, al");
            }

            _ => {}
        }

        println!("\tpush rax");
    }
}



struct LoadError;



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



// Syntax
// expr       = equality
// equality   = relational ("==" relational | "!=" relational)*
// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
// add        = mul ("+" mul | "-" mul)*
// mul        = unary ("*" unary | "/" unary)*
// unary      = ("+" | "-")? primary
// primary    = num | "(" expr ")"


fn consume(tokens: &mut VecDeque<Token>, s: &str) -> bool {
    if tokens.len() > 0 && tokens[0].string == s.to_string() {
        tokens.pop_front();
        return true;
    }

    return false;
}


fn expr(tokens: &mut VecDeque<Token>) -> Rc<Node> {
    return equality(tokens);
}


fn equality(tokens: &mut VecDeque<Token>) -> Rc<Node> {
    let mut node: Rc<Node> = relational(tokens);

    loop {
        if consume(tokens, "==") {
            node = Rc::new(Node {
                kind: NodeKind::Eq,
                lhs: Some(node),
                rhs: Some(relational(tokens)),
            })
        } else if consume(tokens, "!=") {
            node = Rc::new(Node {
                kind: NodeKind::Neq,
                lhs: Some(node),
                rhs: Some(relational(tokens)),
            })
        } else {
            return node;
        }
    }
}


fn relational(tokens: &mut VecDeque<Token>) -> Rc<Node> {
    let mut node: Rc<Node> = add(tokens);

    loop {
        if consume(tokens, "<") {
            node = Rc::new(Node {
                kind: NodeKind::Lt,
                lhs: Some(node),
                rhs: Some(add(tokens)),
            })
        } else if consume(tokens, "<=") {
            node = Rc::new(Node {
                kind: NodeKind::Lte,
                lhs: Some(node),
                rhs: Some(add(tokens)),
            })
        } else if consume(tokens, ">") {
            // Gt: swap lhs and rhs to make it Lt
            node = Rc::new(Node {
                kind: NodeKind::Lt,
                lhs: Some(add(tokens)),
                rhs: Some(node),
            })
        } else if consume(tokens, ">=") {
            // Gte: swap lhs and rhs to make it Lte
            node = Rc::new(Node {
                kind: NodeKind::Lte,
                lhs: Some(add(tokens)),
                rhs: Some(node),
            })
        } else {
            return node;
        }
    }
}


fn add(tokens: &mut VecDeque<Token>) -> Rc<Node> {
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
    let mut node: Rc<Node> = unary(tokens);

    loop {
        loop {
            if consume(tokens, "*") {
                node = Rc::new(Node {
                    kind: NodeKind::Mul,
                    lhs: Some(node),
                    rhs: Some(unary(tokens)),
                })
            } else if consume(tokens, "/") {
                node = Rc::new(Node {
                    kind: NodeKind::Div,
                    lhs: Some(node),
                    rhs: Some(unary(tokens)),
                })
            } else {
                return node;
            }
        }
    }
}


fn unary(tokens: &mut VecDeque<Token>) -> Rc<Node> {
    if consume(tokens, "+") {
        return primary(tokens);
    }

    if consume(tokens, "-") {
        return Rc::new(Node {
            kind: NodeKind::Sub,
            lhs: Some(Rc::new(Node {
                kind: NodeKind::Num { value: 0 },
                lhs: None,
                rhs: None,
            })),
            rhs: Some(primary(tokens)),
        });
    }

    return primary(tokens);
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


fn tokenize(s: &String, symbols: &Vec<String>, ignore: &Vec<String>) -> VecDeque<Token> {
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

        if codes.len() == 0 {
            break;
        }

        println!("{:?}, {:?}", codes, tokens);
        panic!("Failed to tokenize!");
    }

    return tokens;
}


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

    let node: Rc<Node> = expr(&mut tokens); 
    Node::gen(&node);

    println!("\tpop rax");
    println!("\tret");
}
