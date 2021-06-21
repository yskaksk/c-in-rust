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
    //next: Option<&Token>,
    val: Option<u32>,
    str: Vec<char>
}

fn consume(tokens: &mut VecDeque<Token>, op: char) -> bool {
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
        } else if c == '+' || c == '-' {
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
            eprintln!("トークナイズできません")
        }
    }
    return tokens
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("only one arg")
    }
    let chars: Vec<char> = args[1].chars().collect();
    let mut tokens = tokenize(chars);

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    println!("  mov rax, {}", expect_number(&mut tokens).unwrap());

    while !tokens.is_empty() {
        if consume(&mut tokens, '+') {
            println!("  add rax, {}", expect_number(&mut tokens).unwrap());
            continue
        }
        expect(&mut tokens, '-');
        println!("  sub rax, {}", expect_number(&mut tokens).unwrap());
    }

    println!("  ret");
}
