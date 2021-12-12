use std::env;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return print_help();
    }

    match args[1].as_str() {
        "reverse" => {
            if args.len() <= 2 {
                println!("Please provide a word.");
            } else {
                println!("{}", reverse(args[2].as_str()));
            }
        },
        "traits" => traits(),
        _ => println!("Command not recognized.")
    };
}

fn print_help() {
    println!("You must specify a command. Available commands:");
    println!("\treverse\t-- reverse the provided string");
}

fn reverse(input: &str) -> String {
    return UnicodeSegmentation::graphemes(input, true).rev().collect();
}


trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn traits() {
    // Trying out code from https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html
    let answer = 42;
    let maybe_pi = 3.14;
    let v: Vec<&dyn Show> = vec![&answer,&maybe_pi];
    for d in v.iter() {
        println!("show {}",d.show());
    }
}


// Huh! Tests can go right in the Rust file?
#[cfg(test)]
mod tests {
    use super::reverse;

    #[test]
    fn reverse_works_on_empty_string() {
        assert_eq!("", reverse(""));
    }

    #[test]
    fn reverse_works_on_hello_world() {
        assert_eq!("!dlroW ,olleH", reverse("Hello, World!"));
    }

    #[test]
    fn reverse_handles_unicode() {
        assert_eq!("Schüße", reverse("eßühcS"))
    }
    
    #[test]
    fn reverse_handles_zalgo() {
        assert_eq!("i̵̩͙͊̓̃H̷̦̭̱̚", reverse("H̷̦̭̱̚i̵̩͙͊̓̃"))
    }
}
