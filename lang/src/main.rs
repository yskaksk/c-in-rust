use std::env;

fn strtol(chars: &Vec<char>, ind: usize) -> Option<(u32, usize)> {
    let mut i = ind;
    match chars[i].to_digit(10) {
        Some(d) => {
            i = i + 1;
            let mut r: u32 = d;
            while i < chars.len() {
                match chars[i].to_digit(10) {
                    Some(d) => {
                        r = 10 * r + d;
                        i += 1;
                    }
                    None => break,
                }
            }
            return Some((r, i));
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
    match strtol(&chars, ind) {
        Some((r, i)) => {
            ind = i;
            println!("  mov rax, {}", r);
            while ind < chars.len() {
                if chars[ind] == '+' {
                    ind += 1;
                    match strtol(&chars, ind) {
                        Some((r, i)) => {
                            ind = i;
                            println!("  add rax, {}", r)
                        }
                        None => eprintln!("cannot parse at {}", ind),
                    }
                } else if chars[ind] == '-' {
                    ind += 1;
                    match strtol(&chars, ind) {
                        Some((r, i)) => {
                            ind = i;
                            println!("  sub rax, {}", r)
                        }
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
