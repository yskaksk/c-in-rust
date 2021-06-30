use crate::parse::Node;
use crate::parse::Node::*;

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
        ND_FUNCTION {name:_, body:_, stack_size:_} => unreachable!(),
        ND_NOTHING => {}
        ND_RETURN { ret } => {
            gen(*ret, scope_count);
            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
        }
        ND_FUNCALL { name, args } => {
            let mut nargs = 0;
            for arg in args {
                gen(arg, scope_count);
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
