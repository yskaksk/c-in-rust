use std::env;

fn strtol(chars: &Vec<char>, ind: &mut usize) -> Option<u32> {
    match chars[*ind].to_digit(10) {
        Some(d) => {
            *ind += 1;
            let mut r: u32 = d;
            while *ind < chars.len() {
                match chars[*ind].to_digit(10) {
                    Some(d) => {
                        r = 10 * r + d;
                        *ind += 1;
                    }
                    None => break,
                }
            }
            return Some(r);
        }
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("only one arg")
    }
    let chars: Vec<char> = args[1].chars().collect();

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let mut ind = 0;
    match strtol(&chars, &mut ind) {
        Some(r) => {
            println!("  mov rax, {}", r);
            while ind < chars.len() {
                if chars[ind] == '+' {
                    ind += 1;
                    match strtol(&chars, &mut ind) {
                        Some(r) => println!("  add rax, {}", r),
                        None => eprintln!("cannot parse at {}", ind),
                    }
                } else if chars[ind] == '-' {
                    ind += 1;
                    match strtol(&chars, &mut ind) {
                        Some(r) => println!("  sub rax, {}", r),
                        None => eprintln!("cannot parse at {}", ind),
                    }
                } else {
                    eprintln!("unexpected char {} at {}", chars[ind], ind);
                    break;
                }
            }
        }
        None => eprintln!("cannot parse"),
    }

    println!("  ret");
}
