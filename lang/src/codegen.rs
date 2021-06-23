use crate::parse::Token;
use crate::parse::TokenKind::{TK_IDENT, TK_NUM, TK_RESERVED, TK_RETURN};
use std::collections::VecDeque;

#[derive(Clone)]
pub enum Node {
    ND_ADD { lhs: Box<Node>, rhs: Box<Node> },
    ND_SUB { lhs: Box<Node>, rhs: Box<Node> },
    ND_MUL { lhs: Box<Node>, rhs: Box<Node> },
    ND_DIV { lhs: Box<Node>, rhs: Box<Node> },
    ND_ASSIGN { lhs: Box<Node>, rhs: Box<Node> },
    ND_LVAR { offset: u32 },
    ND_NUM(u32),
    ND_EQ { lhs: Box<Node>, rhs: Box<Node> },
    ND_NE { lhs: Box<Node>, rhs: Box<Node> },
    ND_LT { lhs: Box<Node>, rhs: Box<Node> },
    ND_LE { lhs: Box<Node>, rhs: Box<Node> },
    ND_RETURN { ret: Box<Node> },
}

use Node::*;

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

// program = stmt*
pub fn program(tokens: &mut VecDeque<Token>) -> Vec<Node> {
    let mut lvars: VecDeque<LVar> = VecDeque::new();
    let mut nodes: Vec<Node> = Vec::new();
    while !tokens.is_empty() {
        nodes.push(stmt(tokens, &mut lvars));
    }
    return nodes;
}

// stmt = expr ";"
//   | "if" "(" expr ")" stmt ("else" stmt)?
//   | "while" "(" expr ")" stmt
//   | "for" "(" expr? ";" expr? ";" expr? ";" ")" stmt
//   | "return" expr ";"
fn stmt(tokens: &mut VecDeque<Token>, lvars: &mut VecDeque<LVar>) -> Node {
    let node = if let Some(_token) = consume_return(tokens) {
        let expr_node = expr(tokens, lvars);
        ND_RETURN {
            ret: Box::new(expr_node),
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
        node = ND_ASSIGN {
            lhs: Box::new(node),
            rhs: Box::new(assign(tokens, lvars)),
        }
    }
    return node;
}

// equality   = relational ( "==" relational | "!=" relational)*
fn equality(tokens: &mut VecDeque<Token>, lvars: &mut VecDeque<LVar>) -> Node {
    let mut node: Node = relational(tokens, lvars);
    loop {
        if consume(tokens, "==") {
            node = ND_EQ {
                lhs: Box::new(node),
                rhs: Box::new(relational(tokens, lvars)),
            }
        } else if consume(tokens, "!=") {
            node = ND_NE {
                lhs: Box::new(node),
                rhs: Box::new(relational(tokens, lvars)),
            }
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
            node = ND_LT {
                lhs: Box::new(node),
                rhs: Box::new(add(tokens, lvars)),
            }
        } else if consume(tokens, "<=") {
            node = ND_LE {
                lhs: Box::new(node),
                rhs: Box::new(add(tokens, lvars)),
            }
        } else if consume(tokens, ">") {
            node = ND_LT {
                lhs: Box::new(add(tokens, lvars)),
                rhs: Box::new(node),
            }
        } else if consume(tokens, ">=") {
            node = ND_LE {
                lhs: Box::new(add(tokens, lvars)),
                rhs: Box::new(node),
            }
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
            node = ND_ADD {
                lhs: Box::new(node),
                rhs: Box::new(mul(tokens, lvars)),
            }
        } else if consume(tokens, "-") {
            node = ND_SUB {
                lhs: Box::new(node),
                rhs: Box::new(mul(tokens, lvars)),
            }
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
            node = ND_MUL {
                lhs: Box::new(node),
                rhs: Box::new(unary(tokens, lvars)),
            }
        } else if consume(tokens, "/") {
            node = ND_DIV {
                lhs: Box::new(node),
                rhs: Box::new(unary(tokens, lvars)),
            }
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
        return ND_SUB {
            lhs: Box::new(ND_NUM(0)),
            rhs: Box::new(unary(tokens, lvars)),
        };
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
            ND_LVAR {
                offset: lvar.offset,
            }
        } else {
            let offset = match lvars.front() {
                Some(lv) => lv.offset + 8,
                None => 0,
            };
            lvars.push_front(LVar {
                name: token.str,
                offset,
            });
            ND_LVAR { offset }
        };
    }
    return ND_NUM(expect_number(tokens).unwrap());
}

fn gen_lval(node: Node) {
    match node {
        ND_LVAR { offset } => {
            println!("  mov rax, rbp");
            println!("  sub rax, {}", offset);
            println!("  push rax");
        }
        _ => {
            eprintln!("代入の左辺値が変数ではありません");
            std::process::exit(1);
        }
    }
}

fn gen_bin_op(lhs: Node, rhs: Node) {
    gen(lhs);
    gen(rhs);

    println!("  pop rdi");
    println!("  pop rax");
}

pub fn gen(node: Node) {
    match node {
        ND_RETURN { ret } => {
            gen(*ret);
            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
        }
        ND_NUM(val) => {
            println!("  push {}", val);
        }
        ND_LVAR { offset: _ } => {
            gen_lval(node);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
        }
        ND_ASSIGN { lhs, rhs } => {
            gen_lval(*lhs);
            gen(*rhs);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
        }
        ND_ADD { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs);
            println!("  add rax, rdi");
            println!("  push rax");
        }
        ND_SUB { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs);
            println!("  sub rax, rdi");
            println!("  push rax");
        }
        ND_MUL { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs);
            println!("  imul rax, rdi");
            println!("  push rax");
        }
        ND_DIV { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs);
            println!("  cqo");
            println!("  idiv rdi");
            println!("  push rax");
        }
        ND_EQ { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs);
            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        ND_NE { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs);
            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        ND_LT { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs);
            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        ND_LE { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs);
            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
    }
}
