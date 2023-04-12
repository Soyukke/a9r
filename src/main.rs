use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(target) = args.get(1) {
        let cs: Vec<char> = target.chars().collect();
        match cs.len() {
            1 | 2 => println!("{}", target),
            _ => {
                let n = cs.len() - 2;
                println!("{}{}{}", cs[0], n, cs[cs.len()-1])
            }
        }
        if cs.len() < 3 {
        } 
    }
}
