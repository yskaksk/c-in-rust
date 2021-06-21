#![allow(non_camel_case_types)]
use std::env;
use std::collections::VecDeque;

#[derive(Clone)]
enum TokenKind {
    TK_RESERVED,
    TK_NUM,
}

use TokenKind::{TK_RESERVED, TK_NUM};

#[derive(Clone)]
struct Token {
    kind: TokenKind,
    val: Option<u32>,
    str: Vec<char>
}

#[derive(Clone)]
enum NodeKind {
    ND_ADD,
    ND_SUB,
    ND_MUL,
    ND_DIV,
    ND_NUM
}

use NodeKind::{ND_ADD, ND_SUB, ND_MUL, ND_DIV, ND_NUM};

#[derive(Clone)]
struct Node {
    kind: NodeKind,
    lhs: Box<Option<Node>>,
    rhs: Box<Option<Node>>,
    val: Option<u32>
}

fn consume(tokens: &mut VecDeque<Token>, op: char) -> bool {
    if tokens.len() < 1 {
        return false
    }
    let token = tokens.front().unwrap();
    match token.kind {
        TK_RESERVED => {
            if token.str[0] == op {
                tokens.pop_front();
                return true;
            } else {
                return false;
            }
        },
        _ => return false
    }
}

fn expect(tokens: &mut VecDeque<Token>, op: char) {
    let token = tokens.front().unwrap();
    match token.kind {
        TK_RESERVED => {
            if token.str[0] == op {
                tokens.pop_front();
            } else {
                eprintln!("{}ではありません", op)
            }
        },
        _ => eprintln!("{}ではありません", op)
    }
}

fn expect_number(tokens: &mut VecDeque<Token>) -> Result<u32, String> {
    let token = tokens.front().unwrap();
    match token.kind {
        TK_NUM => {
            let val = token.val.unwrap();
            tokens.pop_front();
            return Ok(val)
        },
        _ => Err(String::from("数ではありません"))
    }
}

fn strtol(chars: &Vec<char>, ind: &mut usize) -> Option<u32> {
    match chars[*ind].to_digit(10) {
        Some(d) => {
            *ind += 1;
            let mut r: u32 = d;
            while *ind < chars.len() {
                match chars[*ind].to_digit(10) {
                    Some(d) => {
                        r = 10 * r + d;
                        *ind += 1;
                    }
                    None => break,
                }
            }
            return Some(r);
        }
        _ => None,
    }
}

fn tokenize(chars: Vec<char>) -> VecDeque<Token> {
    let mut tokens = VecDeque::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == ' ' {
            i += 1;
            continue
        } else if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' {
            i += 1;
            let token = Token {
                kind: TK_RESERVED,
                val: None,
                str: vec![c].clone()
            };
            tokens.push_back(token);
            continue
        } else if c.is_digit(10) {
            let token = Token {
                kind: TK_NUM,
                val: strtol(&chars, &mut i),
                str: vec![c].clone(),
            };
            tokens.push_back(token);
            continue
        } else {
            panic!("トークナイズできません");
        }
    }
    return tokens
}

fn new_node(kind: NodeKind, lhs: Node, rhs: Node) -> Node {
    return Node {
        kind,
        lhs: Box::new(Some(lhs)),
        rhs: Box::new(Some(rhs)),
        val: None
    }
}

fn new_node_num(val: u32) -> Node {
    return Node {
        kind: ND_NUM,
        lhs: Box::new(None),
        rhs: Box::new(None),
        val: Some(val)
    }
}

fn expr(tokens: &mut VecDeque<Token>) -> Node {
    let mut node: Node = mul(tokens);
    loop {
        if consume(tokens, '+') {
            node = new_node(ND_ADD, node.clone(), mul(tokens));
        } else if consume(tokens, '-') {
            node = new_node(ND_SUB, node.clone(), mul(tokens));
        } else {
            return node
        }
    }
}

fn mul(tokens: &mut VecDeque<Token>) -> Node {
    let mut node: Node = primary(tokens);
    loop {
        if consume(tokens, '*') {
            node = new_node(ND_MUL, node.clone(), primary(tokens));
        } else if consume(tokens, '/') {
            node = new_node(ND_DIV, node.clone(), primary(tokens));
        } else {
            return node
        }
    }
}

fn primary(tokens: &mut VecDeque<Token>) -> Node {
    if consume(tokens, '(') {
        let node = expr(tokens);
        expect(tokens, ')');
        return node
    } else {
        return new_node_num(expect_number(tokens).unwrap())
    }
}

fn gen(node: Node) {
    match node.kind {
        ND_NUM => {
            println!("  push {}", node.val.unwrap());
            return
        },
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
                },
                _ => unreachable!()
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

    //println!("  mov rax, {}", expect_number(&mut tokens).unwrap());

    //while !tokens.is_empty() {
    //    if consume(&mut tokens, '+') {
    //        println!("  add rax, {}", expect_number(&mut tokens).unwrap());
    //        continue
    //    }
    //    expect(&mut tokens, '-');
    //    println!("  sub rax, {}", expect_number(&mut tokens).unwrap());
    //}

    println!("  pop rax");
    println!("  ret");
}
