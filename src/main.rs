#[macro_use]
extern crate serde_derive;

use std::io;
use std::io::Write;

mod eliza;
mod keywords;
mod farewells;
mod greetings;
mod reflections;
mod synonyms;
mod transforms;
mod script_loader;

use eliza::Eliza;

fn main() {
    println!("ELIZA begin");
    //eliza init -> loads eliza script (could use cmdline arg for script location)

    let eliza = Eliza::new("scripts/rogerian_psychiatrist").expect("Eliza failed to load");
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
            _ => println!("Go on..."), //eliza would create 'intelligent' response to their input
        }
    }

    //eliza farewells the user
    println!();
    println!("{}", eliza.farewell());
}
