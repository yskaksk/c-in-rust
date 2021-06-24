use std::cmp;
use std::collections::VecDeque;

#[derive(Clone, PartialEq, Debug)]
pub enum TokenKind {
    TK_RESERVED,
    TK_IDENT,
    TK_NUM,
    TK_RETURN,
    TK_IF,
    TK_WHILE,
    TK_FOR,
}

use TokenKind::*;

#[derive(Clone, Debug)]
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
        } else if startwith_keyword(&chars, &mut i, "return") {
            tokens.push_back(Token {
                kind: TK_RETURN,
                val: None,
                str: String::from("return"),
            });
            continue;
        } else if startwith_keyword(&chars, &mut i, "for") {
            tokens.push_back(Token {
                kind: TK_FOR,
                val: None,
                str: String::from("for"),
            });
            continue;
        } else if startwith_keyword(&chars, &mut i, "while") {
            tokens.push_back(Token {
                kind: TK_WHILE,
                val: None,
                str: String::from("while"),
            });
            continue;
        } else if startwith_keyword(&chars, &mut i, "if") {
            tokens.push_back(Token {
                kind: TK_IF,
                val: None,
                str: String::from("if"),
            });
            continue;
        } else if startwith_keyword(&chars, &mut i, "else") {
            tokens.push_back(Token {
                kind: TK_RESERVED,
                val: None,
                str: String::from("else"),
            });
            continue;
        } else if let Some(str) = startwith_ident(&chars, &mut i) {
            tokens.push_back(Token {
                kind: TK_IDENT,
                val: None,
                str: str,
            });
            continue;
        } else if let Some(str) = startwith(
            &chars,
            &mut i,
            // 2文字のトークンを先にチェックする
            vec![
                "<=", ">=", "==", "!=", "+", "-", "*", "/", "(", ")", "<", ">", ";", "=",
            ],
        ) {
            tokens.push_back(Token {
                kind: TK_RESERVED,
                val: None,
                str: str,
            });
            continue;
        } else if c.is_digit(10) {
            tokens.push_back(Token {
                kind: TK_NUM,
                val: strtol(&chars, &mut i),
                str: c.to_string(),
            });
            continue;
        } else {
            eprintln!(" {} はトークナイズできません", c);
            std::process::exit(1);
        }
    }
    return tokens;
}

fn startwith_keyword(chars: &Vec<char>, ind: &mut usize, keyword: &str) -> bool {
    let i = ind.clone();
    let klen = keyword.len();
    let sub_chars = &chars[i..cmp::min(i + klen, chars.len())]
        .iter()
        .collect::<String>();
    if sub_chars.to_string() == keyword.to_string() {
        if i + klen == chars.len() || !chars[i + klen].is_ascii_lowercase() {
            *ind += klen;
            return true;
        }
    }
    return false;
}

fn startwith_ident(chars: &Vec<char>, ind: &mut usize) -> Option<String> {
    let mut i = ind.clone();
    let mut char_vec: Vec<char> = Vec::new();
    while chars[i].is_ascii_lowercase() {
        char_vec.push(chars[i]);
        i += 1;
    }
    if char_vec.is_empty() {
        return None;
    }
    *ind += char_vec.len();
    return Some(char_vec.iter().collect());
}

fn startwith(chars: &Vec<char>, ind: &mut usize, patterns: Vec<&str>) -> Option<String> {
    let i = ind.clone();
    for pat in patterns {
        let sub_chars = &chars[i..cmp::min(i + pat.len(), chars.len())]
            .iter()
            .collect::<String>();
        if sub_chars.to_string() == pat.to_string() {
            *ind += sub_chars.len();
            return Some(sub_chars.to_string());
        }
    }
    return None;
}

fn strtol(chars: &Vec<char>, ind: &mut usize) -> Option<u32> {
    if let Some(d) = chars[*ind].to_digit(10) {
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
    return None;
}
