use std::env;

use lang::codegen::gen;
use lang::codegen::program;
use lang::parse::tokenize;

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
    println!(".global main");
    println!("main:");

    // 変数用の領域を確保する
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    for node in nodes {
        gen(node);
        println!("  pop rax");
    }

    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}
