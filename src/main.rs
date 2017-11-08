use std::io;
use std::io::Write;

fn main() {
    println!("ELIZA begin.");
    println!("  Enter '/quit' to close session.");
    //eliza init -> loads eliza script (could use cmdline arg for script location)
    //eliza print greeting

    println!("Welcome!");

    loop {
        print!("> ");
        io::stdout().flush().expect("  Failed to read line.");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("  Failed to read line.");

        match input.as_ref() {
            "/quit\n" => break,
            _ => println!("Go on..."),
        }
    }

    println!("Lets resume next session.");
}
