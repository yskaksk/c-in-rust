use std::cmp;
use std::collections::VecDeque;

#[derive(Clone)]
pub enum TokenKind {
    TK_RESERVED,
    TK_NUM,
}

use TokenKind::{TK_NUM, TK_RESERVED};

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub val: Option<u32>,
    pub str: String,
}

pub fn tokenize(chars: Vec<char>) -> VecDeque<Token> {
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
