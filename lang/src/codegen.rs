use crate::parse::Token;
use crate::parse::TokenKind::{TK_IDENT, TK_NUM, TK_RESERVED, TK_RETURN};
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
    ND_RETURN,
}

use NodeKind::*;

#[derive(Clone)]
pub struct Node {
    kind: NodeKind,
    lhs: Box<Option<Node>>,
    rhs: Box<Option<Node>>,
    val: Option<u32>,
    offset: Option<u32>,
}

#[derive(Clone)]
struct LVar {
    name: String,
    offset: u32,
}

fn find_lvar(token: &Token, lvars: &VecDeque<LVar>) -> Option<LVar> {
    for lvar in lvars {
        if token.str == lvar.name {
            return Some(lvar.clone());
        }
    }
    return None;
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
        return match token.kind {
            TK_IDENT => tokens.pop_front(),
            _ => None,
        };
    }
    return None;
}

fn consume_return(tokens: &mut VecDeque<Token>) -> Option<Token> {
    if let Some(token) = tokens.front() {
        return match token.kind {
            TK_RETURN => tokens.pop_front(),
            _ => None,
        };
    }
    return None;
}

fn expect(tokens: &mut VecDeque<Token>, op: &str) {
    if let Some(token) = tokens.front() {
        match token.kind {
            TK_RESERVED if token.str == op => {
                tokens.pop_front();
                return;
            }
            _ => {
                eprintln!("{}を読み込もうとしましたが、ありませんでした", op);
                std::process::exit(1);
            }
        }
    }
    eprintln!("文末の{}が必要です", op);
    std::process::exit(1);
}

fn expect_number(tokens: &mut VecDeque<Token>) -> Result<u32, String> {
    if let Some(token) = tokens.front() {
        return match token.kind {
            TK_NUM => {
                let val = token.val.unwrap();
                tokens.pop_front();
                Ok(val)
            }
            _ => Err(String::from("数を期待ましたが、数ではありませんでした")),
        };
    }
    return Err(String::from("二項演算子が文末に来ることはありません"));
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
    let mut lvars: VecDeque<LVar> = VecDeque::new();
    let mut nodes: Vec<Node> = Vec::new();
    while !tokens.is_empty() {
        nodes.push(stmt(tokens, &mut lvars));
    }
    return nodes;
}

// stmt = expr ";" | "return" expr ";"
fn stmt(tokens: &mut VecDeque<Token>, lvars: &mut VecDeque<LVar>) -> Node {
    let node = if let Some(_token) = consume_return(tokens) {
        let expr_node = expr(tokens, lvars);
        Node {
            kind: ND_RETURN,
            lhs: Box::new(Some(expr_node)),
            rhs: Box::new(None),
            val: None,
            offset: None,
        }
    } else {
        expr(tokens, lvars)
    };
    expect(tokens, ";");
    return node;
}

// expr = assign
fn expr(tokens: &mut VecDeque<Token>, lvars: &mut VecDeque<LVar>) -> Node {
    return assign(tokens, lvars);
}

// assign = equality ("=" assign)?
fn assign(tokens: &mut VecDeque<Token>, lvars: &mut VecDeque<LVar>) -> Node {
    let mut node: Node = equality(tokens, lvars);
    if consume(tokens, "=") {
        node = new_node(ND_ASSIGN, node, assign(tokens, lvars));
    }
    return node;
}

// equality   = relational ( "==" relational | "!=" relational)*
fn equality(tokens: &mut VecDeque<Token>, lvars: &mut VecDeque<LVar>) -> Node {
    let mut node: Node = relational(tokens, lvars);
    loop {
        if consume(tokens, "==") {
            node = new_node(ND_EQ, node.clone(), relational(tokens, lvars));
        } else if consume(tokens, "!=") {
            node = new_node(ND_NE, node.clone(), relational(tokens, lvars));
        } else {
            return node;
        }
    }
}

// relational = add("<" add | "<=" add | ">" add | ">=" add)*
fn relational(tokens: &mut VecDeque<Token>, lvars: &mut VecDeque<LVar>) -> Node {
    let mut node: Node = add(tokens, lvars);
    loop {
        if consume(tokens, "<") {
            node = new_node(ND_LT, node.clone(), add(tokens, lvars));
        } else if consume(tokens, "<=") {
            node = new_node(ND_LE, node.clone(), add(tokens, lvars));
        } else if consume(tokens, ">") {
            node = new_node(ND_LT, add(tokens, lvars), node.clone());
        } else if consume(tokens, ">=") {
            node = new_node(ND_LE, add(tokens, lvars), node.clone());
        } else {
            return node;
        }
    }
}

// add        = mul ("+" mul | "-" mul)*
fn add(tokens: &mut VecDeque<Token>, lvars: &mut VecDeque<LVar>) -> Node {
    let mut node: Node = mul(tokens, lvars);
    loop {
        if consume(tokens, "+") {
            node = new_node(ND_ADD, node.clone(), mul(tokens, lvars));
        } else if consume(tokens, "-") {
            node = new_node(ND_SUB, node.clone(), mul(tokens, lvars));
        } else {
            return node;
        }
    }
}

// mul        = unary ("*" unary | "/" unary)*
fn mul(tokens: &mut VecDeque<Token>, lvars: &mut VecDeque<LVar>) -> Node {
    let mut node: Node = unary(tokens, lvars);
    loop {
        if consume(tokens, "*") {
            node = new_node(ND_MUL, node.clone(), unary(tokens, lvars));
        } else if consume(tokens, "/") {
            node = new_node(ND_DIV, node.clone(), unary(tokens, lvars));
        } else {
            return node;
        }
    }
}

// unary      = ("+" | "-")? primary
fn unary(tokens: &mut VecDeque<Token>, lvars: &mut VecDeque<LVar>) -> Node {
    if consume(tokens, "+") {
        return unary(tokens, lvars);
    } else if consume(tokens, "-") {
        return new_node(ND_SUB, new_node_num(0), unary(tokens, lvars));
    } else {
        return primary(tokens, lvars);
    }
}

// primary    = num | ident | "(" expr ")"
fn primary(tokens: &mut VecDeque<Token>, lvars: &mut VecDeque<LVar>) -> Node {
    if consume(tokens, "(") {
        let node = expr(tokens, lvars);
        expect(tokens, ")");
        return node;
    } else if let Some(token) = consume_ident(tokens) {
        return if let Some(lvar) = find_lvar(&token, lvars) {
            new_node_lvar(lvar.offset)
        } else {
            let offset = match lvars.front() {
                Some(lv) => lv.offset + 8,
                None => 0,
            };
            lvars.push_front(LVar {
                name: token.str,
                offset,
            });
            new_node_lvar(offset)
        };
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
        ND_RETURN => {
            gen(node.lhs.unwrap());
            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
            return;
        }
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
                ND_NUM | ND_LVAR | ND_ASSIGN | ND_RETURN => unreachable!(),
            }
            println!("  push rax");
        }
    }
}
