#[macro_use] extern crate serde_derive;
extern crate regex;

mod eliza;
mod alphabet;
mod script;

use std::{env, io, process};
use std::io::Write;
use eliza::Eliza;

fn main() {
    //TODO: unit tests
    //TODO: boxed results vs. eprintln

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: `./eliza [SCRIPT]`");
        process::exit(1);
    }

    println!("ELIZA begin");
    let mut eliza = Eliza::new(&args[1]).expect("Eliza failed to load");
    println!("Enter '/quit' to leave the session.\n");

    println!("{}\n", eliza.greet()); //eliza greets the user
    loop {
        print!("> ");
        io::stdout().flush().expect("  Failed to read line.");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("  Failed to read line.");

        match input.as_ref() {
            "/quit\n" => break,
            //Based on the rules in the script, eliza responds to the given input
            _ => println!("{}\n", eliza.respond(&input)),
        }
    }

    println!("\n{}", eliza.farewell()); //eliza farewells the user
}
