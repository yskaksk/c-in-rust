use std::env;

use lang::codegen::gen;
use lang::parse::program;
use lang::parse::Node::ND_FUNCTION;
use lang::tokenize::tokenize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("only one arg");
        std::process::exit(1);
    }
    let chars: Vec<char> = args[1].chars().collect();
    let mut tokens = tokenize(chars);
    let nodes = program(&mut tokens);

    println!(".intel_syntax noprefix");

    let mut scope_count = 0;
    for node in nodes {
        match node {
            ND_FUNCTION {
                name,
                body,
                stack_size,
            } => {
                println!(".global {}", name);
                println!("{}:", name);

                // Prologue
                println!("  push rbp");
                println!("  mov rbp, rsp");
                println!("  sub rsp, {}", stack_size);

                // Emit code
                for bnode in body {
                    gen(bnode, &mut scope_count);
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
