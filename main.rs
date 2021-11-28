use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return print_help();
    }
}

fn print_help() {
    println!("You must specify a command. Available commands:");
    println!("\tsomething");
    println!("\tsomethingelse");
}