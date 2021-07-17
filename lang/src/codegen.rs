use crate::parse::Node;
use crate::parse::Node::*;

fn gen_lval(node: Node, stack_size: u32) {
    match node {
        ND_LVAR { offset } => {
            println!("  lea rax, [rbp-{}]", stack_size - offset);
            println!("  push rax");
        }
        _ => {
            eprintln!("代入の左辺値が変数ではありません");
            std::process::exit(1);
        }
    }
}

fn gen_bin_op(lhs: Node, rhs: Node, scope_count: &mut u32, fname: &String, stack_size: u32) {
    gen(lhs, scope_count, fname, stack_size);
    gen(rhs, scope_count, fname, stack_size);

    println!("  pop rdi");
    println!("  pop rax");
}

pub fn codegen(funcs: Vec<Node>) {
    let mut scope_count = 0;
    let argreg = vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
    for fun in funcs {
        match fun {
            ND_FUNCTION {
                name,
                body,
                parameters,
                stack_size,
            } => {
                println!(".global {}", name);
                println!("{}:", name);

                // Prologue
                println!("  push rbp");
                println!("  mov rbp, rsp");
                println!("  sub rsp, {}", stack_size);

                let mut i = 0;
                for param in parameters {
                    match param {
                        ND_LVAR { offset } => {
                            println!("  mov [rbp-{}], {}", stack_size - offset, argreg[i]);
                            i += 1;
                        }
                        _ => unreachable!(),
                    }
                }

                // Emit code
                for bnode in body {
                    gen(bnode, &mut scope_count, &name, stack_size);
                }
                // Epilogue
                println!(".L.return.{}:", name);
                println!("  mov rsp, rbp");
                println!("  pop rbp");
                println!("  ret");
            }
            _ => unreachable!(),
        }
    }
}

fn gen(node: Node, scope_count: &mut u32, fname: &String, stack_size: u32) {
    let argreg = vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
    match node {
        ND_FUNCTION {
            name: _,
            body: _,
            parameters: _,
            stack_size: _,
        } => unreachable!(),
        ND_NOTHING => {}
        ND_RETURN { ret } => {
            gen(*ret, scope_count, fname, stack_size);
            println!("  pop rax");
            println!("  jmp .L.return.{}", fname);
        }
        ND_FUNCALL { name, args } => {
            let mut nargs = 0;
            for arg in args {
                gen(arg, scope_count, fname, stack_size);
                nargs += 1;
            }
            for i in (0..nargs).rev() {
                println!("  pop {}", argreg[i]);
            }
            // ABIの制約のため、RSPを16の倍数にしておく必要がある
            let sc = scope_count.clone();
            *scope_count += 1;
            println!("  mov rax, rsp");
            // raxの下位４ビットを切り出す
            // 下位４ビットが0←→16の倍数
            println!("  and rax, 15");
            println!("  jnz .L.call.{}", sc);
            println!("  mov rax, 0");
            println!("  call {}", name);
            println!("  jmp .L.end.{}", sc);
            println!(".L.call.{}:", sc);
            println!("  sub rsp, 8");
            println!("  mov rax, 0");
            println!("  call {}", name);
            println!("  add rsp, 8");
            println!(".L.end.{}:", sc);
            println!("  push rax");
        }
        ND_BLOCK { stmts } => {
            for stm in stmts {
                gen(stm, scope_count, fname, stack_size);
                println!("  pop rax");
            }
        }
        ND_IF { cond, cons, alt } => {
            let sc = scope_count.clone();
            *scope_count += 1;
            gen(*cond, scope_count, fname, stack_size);
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je  .Lelse{}", sc);
            gen(*cons, scope_count, fname, stack_size);
            println!("  jmp .Lend{}", sc);
            println!(".Lelse{}:", sc);
            gen(*alt, scope_count, fname, stack_size);
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
            gen(*init, scope_count, fname, stack_size);
            println!(".Lbegin{}:", sc);
            gen(*cond, scope_count, fname, stack_size);
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je  .Lend{}", sc);
            gen(*body, scope_count, fname, stack_size);
            gen(*inc, scope_count, fname, stack_size);
            println!("  jmp  .Lbegin{}", sc);
            println!(".Lend{}:", sc);
        }
        ND_WHILE { cond, body } => {
            let sc = scope_count.clone();
            *scope_count += 1;
            println!(".Lbegin{}:", sc);
            gen(*cond, scope_count, fname, stack_size);
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je  .Lend{}", sc);
            gen(*body, scope_count, fname, stack_size);
            println!("  jmp  .Lbegin{}", sc);
            println!(".Lend{}:", sc);
        }
        ND_NUM(val) => {
            println!("  push {}", val);
        }
        ND_LVAR { offset: _ } => {
            gen_lval(node, stack_size);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
        }
        ND_ASSIGN { lhs, rhs } => {
            gen_lval(*lhs, stack_size);
            gen(*rhs, scope_count, fname, stack_size);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
        }
        ND_ADD { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count, fname, stack_size);
            println!("  add rax, rdi");
            println!("  push rax");
        }
        ND_SUB { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count, fname, stack_size);
            println!("  sub rax, rdi");
            println!("  push rax");
        }
        ND_MUL { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count, fname, stack_size);
            println!("  imul rax, rdi");
            println!("  push rax");
        }
        ND_DIV { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count, fname, stack_size);
            println!("  cqo");
            println!("  idiv rdi");
            println!("  push rax");
        }
        ND_EQ { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count, fname, stack_size);
            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        ND_NE { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count, fname, stack_size);
            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        ND_LT { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count, fname, stack_size);
            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        ND_LE { lhs, rhs } => {
            gen_bin_op(*lhs, *rhs, scope_count, fname, stack_size);
            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
    }
}
