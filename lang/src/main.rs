use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("only one arg")
    }
    let c: u32 = args[1].trim().parse().expect("Please type a number!");

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", c);
    println!("  ret");
}
