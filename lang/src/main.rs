use std::env;

use lang::codegen::codegen;
use lang::parse::program;
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

    codegen(nodes);
}
