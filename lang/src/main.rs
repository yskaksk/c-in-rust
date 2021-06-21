#![allow(non_camel_case_types)]
use std::cmp;
use std::collections::VecDeque;
use std::env;

#[derive(Clone)]
enum TokenKind {
    TK_RESERVED,
    TK_NUM,
}

use TokenKind::{TK_NUM, TK_RESERVED};

#[derive(Clone)]
struct Token {
    kind: TokenKind,
    val: Option<u32>,
    str: String,
}

#[derive(Clone)]
enum NodeKind {
    ND_ADD,
    ND_SUB,
    ND_MUL,
    ND_DIV,
    ND_NUM,
    ND_EQ,
    ND_NE,
    ND_LT,
    ND_LE,
}

use NodeKind::{ND_ADD, ND_DIV, ND_EQ, ND_LE, ND_LT, ND_MUL, ND_NE, ND_NUM, ND_SUB};

#[derive(Clone)]
struct Node {
    kind: NodeKind,
    lhs: Box<Option<Node>>,
    rhs: Box<Option<Node>>,
    val: Option<u32>,
}

fn consume(tokens: &mut VecDeque<Token>, op: &str) -> bool {
    if tokens.len() < 1 {
        return false;
    }
    let token = tokens.front().unwrap();
    match token.kind {
        TK_RESERVED => {
            if token.str == op {
                tokens.pop_front();
                return true;
            } else {
                return false;
            }
        }
        _ => return false,
    }
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

fn strtol(chars: &Vec<char>, ind: &mut usize) -> Option<u32> {
    match chars[*ind].to_digit(10) {
        Some(d) => {
            *ind += 1;
            let mut r: u32 = d;
            while *ind < chars.len() {
                if let Some(d) = chars[*ind].to_digit(10) {
                    r = 10 * r + d;
                    *ind += 1;
                } else {
                    break;
                }
            }
            return Some(r);
        }
        _ => None,
    }
}

fn startwith(chars: &Vec<char>, ind: &mut usize, str: &str) -> Option<String> {
    let i = ind.clone();
    let last = cmp::min(i + str.len(), chars.len());
    let sub_chars = &chars[i..last].iter().collect::<String>();
    if sub_chars.to_string() == str.to_string() {
        *ind += 2;
        return Some(sub_chars.to_string());
    } else {
        return None;
    }
}

fn tokenize(chars: Vec<char>) -> VecDeque<Token> {
    let mut tokens = VecDeque::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == ' ' {
            i += 1;
            continue;
        } else if let Some(str) = startwith(&chars, &mut i, "<=") {
            let token = Token {
                kind: TK_RESERVED,
                val: None,
                str: str,
            };
            tokens.push_back(token);
            continue;
        } else if let Some(str) = startwith(&chars, &mut i, ">=") {
            let token = Token {
                kind: TK_RESERVED,
                val: None,
                str: str,
            };
            tokens.push_back(token);
            continue;
        } else if let Some(str) = startwith(&chars, &mut i, "==") {
            let token = Token {
                kind: TK_RESERVED,
                val: None,
                str: str,
            };
            tokens.push_back(token);
            continue;
        } else if let Some(str) = startwith(&chars, &mut i, "!=") {
            let token = Token {
                kind: TK_RESERVED,
                val: None,
                str: str,
            };
            tokens.push_back(token);
            continue;
        } else if String::from("+-=*/()<>").contains(c) {
            i += 1;
            let token = Token {
                kind: TK_RESERVED,
                val: None,
                str: c.to_string(),
            };
            tokens.push_back(token);
            continue;
        } else if c.is_digit(10) {
            let token = Token {
                kind: TK_NUM,
                val: strtol(&chars, &mut i),
                str: c.to_string(),
            };
            tokens.push_back(token);
            continue;
        } else {
            panic!("トークナイズできません");
        }
    }
    return tokens;
}

fn new_node(kind: NodeKind, lhs: Node, rhs: Node) -> Node {
    return Node {
        kind,
        lhs: Box::new(Some(lhs)),
        rhs: Box::new(Some(rhs)),
        val: None,
    };
}

fn new_node_num(val: u32) -> Node {
    return Node {
        kind: ND_NUM,
        lhs: Box::new(None),
        rhs: Box::new(None),
        val: Some(val),
    };
}

// expr       = equality
fn expr(tokens: &mut VecDeque<Token>) -> Node {
    return equality(tokens);
    //let mut node: Node = mul(tokens);
    //loop {
    //    if consume(tokens, '+') {
    //        node = new_node(ND_ADD, node.clone(), mul(tokens));
    //    } else if consume(tokens, '-') {
    //        node = new_node(ND_SUB, node.clone(), mul(tokens));
    //    } else {
    //        return node;
    //    }
    //}
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

// primary    = num | "(" expr ")"
fn primary(tokens: &mut VecDeque<Token>) -> Node {
    if consume(tokens, "(") {
        let node = expr(tokens);
        expect(tokens, ")");
        return node;
    }
    return new_node_num(expect_number(tokens).unwrap());
}

fn gen(node: Node) {
    match node.kind {
        ND_NUM => {
            println!("  push {}", node.val.unwrap());
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
                _ => unreachable!(),
            }
            println!("  push rax");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("only one arg")
    }
    let chars: Vec<char> = args[1].chars().collect();
    let mut tokens = tokenize(chars);
    let node = expr(&mut tokens);

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    gen(node);

    println!("  pop rax");
    println!("  ret");
}
