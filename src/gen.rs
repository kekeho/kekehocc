use std::collections::{VecDeque, HashMap};
use std::rc::Rc;


use crate::tokenize::{Token, TokenKind};


#[derive(PartialEq, Debug)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Lt,
    Lte,
    Assign,
    LocalVar {offset: usize},
    Num {value: i64},
}

#[derive(PartialEq, Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Rc<Node>>,
    pub rhs: Option<Rc<Node>>,
}

impl Node {
    pub fn gen(n: &Rc<Node>) {
        match n.kind {
            NodeKind::Num {value: v} => {
                println!("\tpush {}", v);
                return
            }

            NodeKind::LocalVar { offset: _ } => {
                Self::gen_leftval(&n);
                println!("\tpop rax");
                println!("\tmov rax, [rax]");
                println!("\tpush rax");
                return;
            }

            NodeKind::Assign => {
                Self::gen_leftval(&n.lhs.clone().unwrap());
                Self::gen(&n.rhs.clone().unwrap());

                println!("\tpop rdi");
                println!("\tpop rax");
                println!("\tmov [rax], rdi");
                println!("\tpush rdi");
                return;
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

    fn gen_leftval(n: &Rc<Node>) {
        match n.kind {
            NodeKind::LocalVar { offset } => {
                println!("\tmov rax, rbp");
                println!("\tsub rax, {}", offset);
                println!("\tpush rax");
            }

            _ => {
                panic!("Left value is not identifier...");
            }
        }
    }
}


fn consume(tokens: &mut VecDeque<Token>, s: &str) -> bool {
    if tokens.len() > 0 && tokens[0].string == s.to_string() {
        tokens.pop_front();
        return true;
    }
    return false;
}


fn consume_ident(tokens: &mut VecDeque<Token>) -> Option<Token> {
    if tokens.len() > 0 && tokens[0].kind == TokenKind::Ident {
        return tokens.pop_front();
    }

    return None;
}


pub fn program(tokens: &mut VecDeque<Token>) -> (Vec<Rc<Node>>, usize) {
    // Return: Nodes, Local variable count
    let mut nodes: Vec<Rc<Node>> = Vec::new();
    let mut local_vars: HashMap<String, usize> = HashMap::new();
    loop {
        let n = stmt(tokens, &mut local_vars);
        nodes.push(n);
        
        if tokens.len() == 0 {
            break;
        }
    }
    return (nodes, local_vars.len());
}


fn stmt(tokens: &mut VecDeque<Token>, local_vars: &mut HashMap<String, usize>) -> Rc<Node> {
    let n: Rc<Node> = expr(tokens, local_vars);
    consume(tokens, ";");
    return n;
}


fn expr(tokens: &mut VecDeque<Token>, local_vars: &mut HashMap<String, usize>) -> Rc<Node> {
    return assign(tokens, local_vars);
}

fn assign(tokens: &mut VecDeque<Token>, local_vars: &mut HashMap<String, usize>) -> Rc<Node> {
    let n: Rc<Node> = equality(tokens, local_vars);
    if consume(tokens, "=") {
        return Rc::new(Node {
            kind: NodeKind::Assign,
            lhs: Some(n),
            rhs: Some(assign(tokens, local_vars)),
        })
    }
    return n;
}


fn equality(tokens: &mut VecDeque<Token>, local_vars: &mut HashMap<String, usize>) -> Rc<Node> {
    let mut node: Rc<Node> = relational(tokens, local_vars);

    loop {
        if consume(tokens, "==") {
            node = Rc::new(Node {
                kind: NodeKind::Eq,
                lhs: Some(node),
                rhs: Some(relational(tokens, local_vars)),
            })
        } else if consume(tokens, "!=") {
            node = Rc::new(Node {
                kind: NodeKind::Neq,
                lhs: Some(node),
                rhs: Some(relational(tokens, local_vars)),
            })
        } else {
            return node;
        }
    }
}


fn relational(tokens: &mut VecDeque<Token>, local_vars: &mut HashMap<String, usize>) -> Rc<Node> {
    let mut node: Rc<Node> = add(tokens, local_vars);

    loop {
        if consume(tokens, "<") {
            node = Rc::new(Node {
                kind: NodeKind::Lt,
                lhs: Some(node),
                rhs: Some(add(tokens, local_vars)),
            })
        } else if consume(tokens, "<=") {
            node = Rc::new(Node {
                kind: NodeKind::Lte,
                lhs: Some(node),
                rhs: Some(add(tokens, local_vars)),
            })
        } else if consume(tokens, ">") {
            // Gt: swap lhs and rhs to make it Lt
            node = Rc::new(Node {
                kind: NodeKind::Lt,
                lhs: Some(add(tokens, local_vars)),
                rhs: Some(node),
            })
        } else if consume(tokens, ">=") {
            // Gte: swap lhs and rhs to make it Lte
            node = Rc::new(Node {
                kind: NodeKind::Lte,
                lhs: Some(add(tokens, local_vars)),
                rhs: Some(node),
            })
        } else {
            return node;
        }
    }
}


fn add(tokens: &mut VecDeque<Token>, local_vars: &mut HashMap<String, usize>) -> Rc<Node> {
    let mut node: Rc<Node> = mul(tokens, local_vars);
    
    loop {
        if consume(tokens, "+") {
            node = Rc::new(Node {
                kind: NodeKind::Add,
                lhs: Some(node),
                rhs: Some(mul(tokens, local_vars)),
            })
        } else if consume(tokens, "-") {
            node = Rc::new(Node {
                kind: NodeKind::Sub,
                lhs: Some(node),
                rhs: Some(mul(tokens, local_vars)),
            })
        } else {
            return node;
        }
    }
}


fn mul(tokens: &mut VecDeque<Token>, local_vars: &mut HashMap<String, usize>) -> Rc<Node> {
    let mut node: Rc<Node> = unary(tokens, local_vars);

    loop {
        loop {
            if consume(tokens, "*") {
                node = Rc::new(Node {
                    kind: NodeKind::Mul,
                    lhs: Some(node),
                    rhs: Some(unary(tokens, local_vars)),
                })
            } else if consume(tokens, "/") {
                node = Rc::new(Node {
                    kind: NodeKind::Div,
                    lhs: Some(node),
                    rhs: Some(unary(tokens, local_vars)),
                })
            } else {
                return node;
            }
        }
    }
}


fn unary(tokens: &mut VecDeque<Token>, local_vars: &mut HashMap<String, usize>) -> Rc<Node> {
    if consume(tokens, "+") {
        return primary(tokens, local_vars);
    }

    if consume(tokens, "-") {
        return Rc::new(Node {
            kind: NodeKind::Sub,
            lhs: Some(Rc::new(Node {
                kind: NodeKind::Num { value: 0 },
                lhs: None,
                rhs: None,
            })),
            rhs: Some(primary(tokens, local_vars)),
        });
    }

    return primary(tokens, local_vars);
}


fn primary(tokens: &mut VecDeque<Token>, local_vars: &mut HashMap<String, usize>) -> Rc<Node> {
    if consume(tokens, "(") {
        let node: Rc<Node> = expr(tokens, local_vars);
        consume(tokens, ")");
        return node;
    }

    if let Some(t) = consume_ident(tokens) {
        let offset: usize;
        match local_vars.get(&t.string) {
            Some(i) => {
                offset = *i;
            }

            None => {
                offset = (local_vars.len() + 1) * 8;
                local_vars.insert(t.string, offset);
            }
        }

        return Rc::new(Node {
            kind: NodeKind::LocalVar { offset: offset },
            lhs: None,
            rhs: None,
        })
    }

    return Rc::new(Node {
        kind: NodeKind::Num { value: tokens.pop_front().unwrap().val.unwrap() },
        lhs: None,
        rhs: None,
    })
}
