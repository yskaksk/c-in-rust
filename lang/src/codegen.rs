use crate::parse::Token;
use crate::parse::TokenKind::{TK_IDENT, TK_NUM, TK_RESERVED};
use std::collections::VecDeque;

#[derive(Clone)]
enum NodeKind {
    ND_ADD,
    ND_SUB,
    ND_MUL,
    ND_DIV,
    ND_ASSIGN,
    ND_LVAR,
    ND_NUM,
    ND_EQ,
    ND_NE,
    ND_LT,
    ND_LE,
}

use NodeKind::{
    ND_ADD, ND_ASSIGN, ND_DIV, ND_EQ, ND_LE, ND_LT, ND_LVAR, ND_MUL, ND_NE, ND_NUM, ND_SUB,
};

#[derive(Clone)]
pub struct Node {
    kind: NodeKind,
    lhs: Box<Option<Node>>,
    rhs: Box<Option<Node>>,
    val: Option<u32>,
    offset: Option<u32>,
}

fn consume(tokens: &mut VecDeque<Token>, op: &str) -> bool {
    if let Some(token) = tokens.front() {
        match token.kind {
            TK_RESERVED if token.str == op => {
                tokens.pop_front();
                return true;
            }
            _ => return false,
        }
    }
    return false;
}

fn consume_ident(tokens: &mut VecDeque<Token>) -> Option<Token> {
    if let Some(token) = tokens.front() {
        match token.kind {
            TK_IDENT if "abcdefghijklmnopqrstuvwxyz".contains(&token.str) => {
                return tokens.pop_front()
            }
            _ => return None,
        }
    }
    return None;
}

fn expect(tokens: &mut VecDeque<Token>, op: &str) {
    let token = tokens.front().unwrap();
    match token.kind {
        TK_RESERVED => {
            if token.str == op {
                tokens.pop_front();
            } else {
                eprintln!("{}ではありません", op)
            }
        }
        _ => eprintln!("{}ではありません", op),
    }
}

fn expect_number(tokens: &mut VecDeque<Token>) -> Result<u32, String> {
    let token = tokens.front().unwrap();
    match token.kind {
        TK_NUM => {
            let val = token.val.unwrap();
            tokens.pop_front();
            return Ok(val);
        }
        _ => Err(String::from("数ではありません")),
    }
}

fn new_node(kind: NodeKind, lhs: Node, rhs: Node) -> Node {
    return Node {
        kind,
        lhs: Box::new(Some(lhs)),
        rhs: Box::new(Some(rhs)),
        val: None,
        offset: None,
    };
}

fn new_node_num(val: u32) -> Node {
    return Node {
        kind: ND_NUM,
        lhs: Box::new(None),
        rhs: Box::new(None),
        val: Some(val),
        offset: None,
    };
}

fn new_node_lvar(offset: u32) -> Node {
    return Node {
        kind: ND_LVAR,
        lhs: Box::new(None),
        rhs: Box::new(None),
        val: None,
        offset: Some(offset),
    };
}

// program = stmt*
pub fn program(tokens: &mut VecDeque<Token>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    while !tokens.is_empty() {
        nodes.push(stmt(tokens));
    }
    return nodes;
}

// stmt = expr ";"
fn stmt(tokens: &mut VecDeque<Token>) -> Node {
    let node: Node = expr(tokens);
    expect(tokens, ";");
    return node;
}

// expr = assign
fn expr(tokens: &mut VecDeque<Token>) -> Node {
    return assign(tokens);
}

// assign = equality ("=" assign)?
fn assign(tokens: &mut VecDeque<Token>) -> Node {
    let mut node: Node = equality(tokens);
    if consume(tokens, "=") {
        node = new_node(ND_ASSIGN, node, assign(tokens));
    }
    return node;
}

// equality   = relational ( "==" relational | "!=" relational)*
fn equality(tokens: &mut VecDeque<Token>) -> Node {
    let mut node: Node = relational(tokens);
    loop {
        if consume(tokens, "==") {
            node = new_node(ND_EQ, node.clone(), relational(tokens));
        } else if consume(tokens, "!=") {
            node = new_node(ND_NE, node.clone(), relational(tokens));
        } else {
            return node;
        }
    }
}

// relational = add("<" add | "<=" add | ">" add | ">=" add)*
fn relational(tokens: &mut VecDeque<Token>) -> Node {
    let mut node: Node = add(tokens);
    loop {
        if consume(tokens, "<") {
            node = new_node(ND_LT, node.clone(), add(tokens));
        } else if consume(tokens, "<=") {
            node = new_node(ND_LE, node.clone(), add(tokens));
        } else if consume(tokens, ">") {
            node = new_node(ND_LT, add(tokens), node.clone());
        } else if consume(tokens, ">=") {
            node = new_node(ND_LE, add(tokens), node.clone());
        } else {
            return node;
        }
    }
}

// add        = mul ("+" mul | "-" mul)*
fn add(tokens: &mut VecDeque<Token>) -> Node {
    let mut node: Node = mul(tokens);
    loop {
        if consume(tokens, "+") {
            node = new_node(ND_ADD, node.clone(), mul(tokens));
        } else if consume(tokens, "-") {
            node = new_node(ND_SUB, node.clone(), mul(tokens));
        } else {
            return node;
        }
    }
}

// mul        = unary ("*" unary | "/" unary)*
fn mul(tokens: &mut VecDeque<Token>) -> Node {
    let mut node: Node = unary(tokens);
    loop {
        if consume(tokens, "*") {
            node = new_node(ND_MUL, node.clone(), unary(tokens));
        } else if consume(tokens, "/") {
            node = new_node(ND_DIV, node.clone(), unary(tokens));
        } else {
            return node;
        }
    }
}

// unary      = ("+" | "-")? primary
fn unary(tokens: &mut VecDeque<Token>) -> Node {
    if consume(tokens, "+") {
        return unary(tokens);
    } else if consume(tokens, "-") {
        return new_node(ND_SUB, new_node_num(0), unary(tokens));
    } else {
        return primary(tokens);
    }
}

// primary    = num | ident | "(" expr ")"
fn primary(tokens: &mut VecDeque<Token>) -> Node {
    if consume(tokens, "(") {
        let node = expr(tokens);
        expect(tokens, ")");
        return node;
    } else if let Some(token) = consume_ident(tokens) {
        let chars: Vec<char> = token.str.chars().collect();
        let offset = (chars[0] as u32 - 'a' as u32 + 1) * 8;
        return new_node_lvar(offset);
    }
    return new_node_num(expect_number(tokens).unwrap());
}

fn gen_lval(node: Node) {
    match node.kind {
        ND_LVAR => {
            println!("  mov rax, rbp");
            println!("  sub rax, {}", node.offset.unwrap());
            println!("  push rax");
        }
        _ => {
            eprintln!("代入の左辺値が変数ではありません");
            std::process::exit(1);
        }
    }
}

pub fn gen(node: Node) {
    match node.kind {
        ND_NUM => {
            println!("  push {}", node.val.unwrap());
            return;
        }
        ND_LVAR => {
            gen_lval(node);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return;
        }
        ND_ASSIGN => {
            gen_lval(node.lhs.unwrap());
            gen(node.rhs.unwrap());

            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
            return;
        }
        _ => {
            gen(node.lhs.unwrap());
            gen(node.rhs.unwrap());

            println!("  pop rdi");
            println!("  pop rax");

            match node.kind {
                ND_ADD => println!("  add rax, rdi"),
                ND_SUB => println!("  sub rax, rdi"),
                ND_MUL => println!("  imul rax, rdi"),
                ND_DIV => {
                    println!("  cqo");
                    println!("  idiv rdi");
                }
                ND_EQ => {
                    println!("  cmp rax, rdi");
                    println!("  sete al");
                    println!("  movzb rax, al");
                }
                ND_NE => {
                    println!("  cmp rax, rdi");
                    println!("  setne al");
                    println!("  movzb rax, al");
                }
                ND_LT => {
                    println!("  cmp rax, rdi");
                    println!("  setl al");
                    println!("  movzb rax, al");
                }
                ND_LE => {
                    println!("  cmp rax, rdi");
                    println!("  setle al");
                    println!("  movzb rax, al");
                }
                ND_NUM | ND_LVAR | ND_ASSIGN => unreachable!(),
            }
            println!("  push rax");
        }
    }
}
