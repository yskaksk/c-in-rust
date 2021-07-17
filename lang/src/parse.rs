use crate::tokenize::TokenKind::{
    TK_FOR, TK_IDENT, TK_IF, TK_NUM, TK_RESERVED, TK_RETURN, TK_WHILE,
};
use crate::tokenize::{Token, TokenKind};
use std::collections::VecDeque;

#[derive(Clone, Debug)]
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
    ND_FUNCALL {
        name: String,
        args: Vec<Node>,
    },
    ND_FUNCTION {
        name: String,
        body: Vec<Node>,
        parameters: VecDeque<Node>,
        stack_size: u32,
    },
}

use Node::*;

#[derive(Clone, Debug)]
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

// program = function*
pub fn program(tokens: &mut VecDeque<Token>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    while !tokens.is_empty() {
        nodes.push(function(tokens));
    }
    return nodes;
}

// function = ident "(" params? ")" "{" stmt* "}"
fn function(tokens: &mut VecDeque<Token>) -> Node {
    let mut lvars: VecDeque<LVar> = VecDeque::new();
    if let Some(token) = consume_tk(tokens, TK_IDENT) {
        expect(tokens, "(");
        let parameters = params(tokens, &mut lvars);
        expect(tokens, ")");
        expect(tokens, "{");
        let mut body: Vec<Node> = Vec::new();
        while !consume(tokens, "}") {
            body.push(stmt(tokens, &mut lvars));
        }
        let stack_size = 8 * lvars.len() as u32;
        return ND_FUNCTION {
            name: token.str,
            body,
            parameters,
            stack_size,
        };
    } else {
        eprintln!("関数名を期待しましたが、ありませんでした");
        std::process::exit(1);
    }
}

// params   = ident ("," ident)*
fn params(tokens: &mut VecDeque<Token>, lvars: &mut VecDeque<LVar>) -> VecDeque<Node> {
    let mut prms: VecDeque<Node> = VecDeque::new();
    if let Some(token) = consume_tk(tokens, TK_IDENT) {
        prms.push_back(local_var(token, lvars));
        while consume(tokens, ",") {
            if let Some(token) = consume_tk(tokens, TK_IDENT) {
                prms.push_back(local_var(token, lvars))
            }
        }
        return prms;
    } else {
        return prms;
    }
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
            return local_var(token, lvars);
        }
    }
    return ND_NUM(expect_number(tokens).unwrap());
}

fn local_var(token: Token, lvars: &mut VecDeque<LVar>) -> Node {
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
            name: token.str.clone(),
            offset,
        });
        ND_LVAR { offset }
    };
}
