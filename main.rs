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
