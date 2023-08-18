mod query;

use query::parser::Parser;

fn main() {
    println!("Welcome to KavyDB! Type in your commands below.");
    loop {
        let mut input = String::new();

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(error) => {
                println!("failed to read from stdin: {}", error);
                break;
            }
        }

        let mut parser = Parser::new(input);
    }
}
