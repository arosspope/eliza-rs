#[macro_use]
extern crate serde_derive;

use std::io;
use std::io::Write;

mod keywords;
mod messages;

use keywords::Keywords;
use messages::Messages;


fn main() {
    println!("ELIZA begin.");
    println!("  Enter '/quit' to close session.");
    //eliza init -> loads eliza script (could use cmdline arg for script location)
    //eliza greets the user


    let kws = Keywords::load("scripts/rogerian_psychiatrist").unwrap();
    let greetings = Messages::load("scripts/rogerian_psychiatrist", "greetings.json").unwrap();
    let farewells = Messages::load("scripts/rogerian_psychiatrist", "farewells.json").unwrap();

    // let t = match result {
    //     Ok(t) => result,
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     },
    // };

    if let Some(greeting) = greetings.random() {
        println!("{}", greeting);
    } else {
        println!("Howdy!");
    }

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
    if let Some(farewell) = farewells.random() {
        println!("{}", farewell);
    } else {
        println!("Bye!");
    }
}
