#[macro_use] extern crate serde_derive;
extern crate regex;

use std::io;
use std::io::Write;

mod eliza;
// mod keywords;
// mod farewells;
// mod greetings;
// mod fallbacks;
// mod reflections;
// mod synonyms;
// mod transforms;
//mod script_loader;
mod alphabet;
mod script;

use eliza::Eliza;

fn main() {
    //TODO: unit tests
    //TODO: boxed results vs. eprintln

    println!("ELIZA begin");
    //eliza init -> loads eliza script (could use cmdline arg for script location)

    let mut eliza = Eliza::new("scripts/rogerian_psychiatrist.json").expect("Eliza failed to load");
    println!();
    println!("Enter '/quit' to leave the session.");

    println!();
    println!("{}", eliza.greet()); //eliza greets the user

    loop {
        print!("> ");
        io::stdout().flush().expect("  Failed to read line.");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("  Failed to read line.");

        match input.as_ref() {
            "/quit\n" => break,
            //Based on the rules in the script, eliza responds to the given input
            _ => println!("{}", eliza.respond(&input)),
        }
    }

    //eliza farewells the user
    println!();
    println!("{}", eliza.farewell());
}
