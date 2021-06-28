use crate::parse::TokenKind::{TK_FOR, TK_IDENT, TK_IF, TK_NUM, TK_RESERVED, TK_RETURN, TK_WHILE};
use crate::parse::{Token, TokenKind};
use std::collections::VecDeque;

#[derive(Clone)]
pub enum Node {
    ND_NOTHING,
    ND_ADD {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    ND_SUB {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    ND_MUL {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    ND_DIV {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    ND_ASSIGN {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    ND_LVAR {
        offset: u32,
    },
    ND_NUM(u32),
    ND_EQ {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    ND_NE {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    ND_LT {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    ND_LE {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    ND_RETURN {
        ret: Box<Node>,
    },
    ND_IF {
        cond: Box<Node>,
        cons: Box<Node>,
        alt: Box<Node>,
    },
    ND_WHILE {
        cond: Box<Node>,
        body: Box<Node>,
    },
    ND_BLOCK {
        stmts: Vec<Node>,
    },
    ND_FOR {
        init: Box<Node>,
        cond: Box<Node>,
        inc: Box<Node>,
        body: Box<Node>,
    },
<<<<<<< HEAD
    ND_FUNCALL {
        name: String,
        args: Vec<Node>,
=======
    ND_FUNCTION {
        name: String,
>>>>>>> main
    },
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

fn consume_tk(tokens: &mut VecDeque<Token>, tkind: TokenKind) -> Option<Token> {
    if let Some(token) = tokens.front() {
        if token.kind == tkind {
            return tokens.pop_front();
        }
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
//   | "{" stmt* "}"
//   | "if" "(" expr ")" stmt ("else" stmt)?
//   | "while" "(" expr ")" stmt
//   | "for" "(" expr? ";" expr? ";" expr? ";" ")" stmt
//   | "return" expr ";"
fn stmt(tokens: &mut VecDeque<Token>, lvars: &mut VecDeque<LVar>) -> Node {
    let node = if let Some(_token) = consume_tk(tokens, TK_RETURN) {
        let expr_node = expr(tokens, lvars);
        expect(tokens, ";");
        ND_RETURN {
            ret: Box::new(expr_node),
        }
    } else if let Some(_token) = consume_tk(tokens, TK_IF) {
        expect(tokens, "(");
        let cond = Box::new(expr(tokens, lvars));
        expect(tokens, ")");
        let cons = Box::new(stmt(tokens, lvars));
        let alt = Box::new(if consume(tokens, "else") {
            stmt(tokens, lvars)
        } else {
            ND_NOTHING
        });
        ND_IF { cond, cons, alt }
    } else if let Some(_token) = consume_tk(tokens, TK_WHILE) {
        expect(tokens, "(");
        let cond = Box::new(expr(tokens, lvars));
        expect(tokens, ")");
        let body = Box::new(stmt(tokens, lvars));
        ND_WHILE { cond, body }
    } else if consume(tokens, "{") {
        let mut stmts: Vec<Node> = Vec::new();
        while !consume(tokens, "}") {
            stmts.push(stmt(tokens, lvars));
        }
        ND_BLOCK { stmts }
    } else if let Some(_token) = consume_tk(tokens, TK_FOR) {
        expect(tokens, "(");
        let init = Box::new(if consume(tokens, ";") {
            ND_NOTHING
        } else {
            let nd = expr(tokens, lvars);
            expect(tokens, ";");
            nd
        });
        let cond = Box::new(if consume(tokens, ";") {
            ND_NOTHING
        } else {
            let nd = expr(tokens, lvars);
            expect(tokens, ";");
            nd
        });
        let inc = Box::new(if consume(tokens, ";") {
            ND_NOTHING
        } else {
            let nd = expr(tokens, lvars);
            expect(tokens, ";");
            nd
        });
        expect(tokens, ")");
        let body = Box::new(stmt(tokens, lvars));
        ND_FOR {
            init,
            cond,
            inc,
            body,
        }
    } else {
        let nd = expr(tokens, lvars);
        expect(tokens, ";");
        nd
    };
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

// func-args = "(" (assign ("," assign)*)? ")"
fn func_args(tokens: &mut VecDeque<Token>, lvars: &mut VecDeque<LVar>) -> Vec<Node> {
    if consume(tokens, ")") {
        return Vec::new();
    }
    let mut args: Vec<Node> = vec![assign(tokens, lvars)];
    while consume(tokens, ",") {
        args.push(assign(tokens, lvars));
    }
    expect(tokens, ")");
    return args;
}

// primary    = num | ident func-args? | "(" expr ")"
fn primary(tokens: &mut VecDeque<Token>, lvars: &mut VecDeque<LVar>) -> Node {
    if consume(tokens, "(") {
        let node = expr(tokens, lvars);
        expect(tokens, ")");
        return node;
    } else if let Some(token) = consume_tk(tokens, TK_IDENT) {
        if consume(tokens, "(") {
            let args = func_args(tokens, lvars);
            return ND_FUNCALL {
                name: token.str,
                args,
            };
        } else {
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

fn gen_bin_op(lhs: Node, rhs: Node, scope_count: &mut u32) {
    gen(lhs, scope_count);
    gen(rhs, scope_count);

    println!("  pop rdi");
    println!("  pop rax");
}

pub fn gen(node: Node, scope_count: &mut u32) {
    let argreg = vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
    match node {
        ND_NOTHING => {}
        ND_RETURN { ret } => {
            gen(*ret, scope_count);
            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
        }
<<<<<<< HEAD
        ND_FUNCALL { name, args } => {
            let mut nargs = 0;
            for arg in args {
                gen(arg, scope_count);
                nargs += 1;
            }
            for i in (0..nargs).rev() {
                println!("  pop {}", argreg[i]);
            }
=======
        ND_FUNCTION { name } => {
>>>>>>> main
            println!("  call {}", name);
            println!("  push rax");
        }
        ND_BLOCK { stmts } => {
            for stm in stmts {
                gen(stm, scope_count);
                println!("  pop rax");
            }
        }
        ND_IF { cond, cons, alt } => {
            let sc = scope_count.clone();
            *scope_count += 1;
            gen(*cond, scope_count);
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je  .Lelse{}", sc);
            gen(*cons, scope_count);
            println!("  jmp .Lend{}", sc);
            println!(".Lelse{}:", sc);
            gen(*alt, scope_count);
            println!(".Lend{}:", sc);
        }
        ND_FOR {
            init,
            cond,
            inc,
            body,
        } => {
            let sc = scope_count.clone();
            *scope_count += 1;
            gen(*init, scope_count);
            println!(".Lbegin{}:", sc);
            gen(*cond, scope_count);
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je  .Lend{}", sc);
            gen(*body, scope_count);
            gen(*inc, scope_count);
            println!("  jmp  .Lbegin{}", sc);
            println!(".Lend{}:", sc);
        }
        ND_WHILE { cond, body } => {
            let sc = scope_count.clone();
            *scope_count += 1;
            println!(".Lbegin{}:", sc);
            gen(*cond, scope_count);
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je  .Lend{}", sc);
            gen(*body, scope_count);
            println!("  jmp  .Lbegin{}", sc);
            println!(".Lend{}:", sc);
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
            gen(*rhs, scope_count);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
        }
        ND_ADD { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count);
            println!("  add rax, rdi");
            println!("  push rax");
        }
        ND_SUB { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count);
            println!("  sub rax, rdi");
            println!("  push rax");
        }
        ND_MUL { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count);
            println!("  imul rax, rdi");
            println!("  push rax");
        }
        ND_DIV { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count);
            println!("  cqo");
            println!("  idiv rdi");
            println!("  push rax");
        }
        ND_EQ { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count);
            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        ND_NE { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count);
            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        ND_LT { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count);
            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        ND_LE { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count);
            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
    }
}
