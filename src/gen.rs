use std::collections::VecDeque;
use std::rc::Rc;


use crate::tokenize::Token;


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


fn consume(tokens: &mut VecDeque<Token>, s: &str) -> bool {
    if tokens.len() > 0 && tokens[0].string == s.to_string() {
        tokens.pop_front();
        return true;
    }
    return false;
}


pub fn expr(tokens: &mut VecDeque<Token>) -> Rc<Node> {
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
