use std::env;
use hq9p;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("1 argument expected, not {}", args.len()-1)
    }

    hq9p::run(&*args[1])
}
