use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: hello <name>");
        process::exit(1);
    }

    let name = &args[1];
    println!("Hello, {}!", name);
}
