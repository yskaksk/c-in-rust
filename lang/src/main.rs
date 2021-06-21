use std::env;

use lang::codegen::expr;
use lang::codegen::gen;
use lang::parse::tokenize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("only one arg");
        std::process::exit(1);
    }
    let chars: Vec<char> = args[1].chars().collect();
    let mut tokens = tokenize(chars);
    let node = expr(&mut tokens);

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    gen(node);

    println!("  pop rax");
    println!("  ret");
}
