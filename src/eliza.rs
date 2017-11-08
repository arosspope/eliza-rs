use std::error::Error;

use script_loader::ScriptLoader;
use reflections::Reflections;
use keywords::Keywords;
use greetings::Greetings;
use farewells::Farewells;
use synonyms::Synonyms;
use transforms::Transforms;

pub struct Eliza {
    greetings : Greetings,
    farewells : Farewells,
    kwords : Keywords,
    transforms : Transforms,
    synonyms : Synonyms,
    reflections : Reflections,
}

impl Eliza {
    pub fn new(script_location: &str) -> Result<Eliza, Box<Error>> {
        //TODO: Perhaps these prints would be better as debug output
        let e = Eliza {
            greetings: {
                println!("  Loading greetings...");
                Greetings::load(script_location)?
            },
            farewells: {
                println!("  Loading farewells...");
                Farewells::load(script_location)?
            },
            kwords: {
                println!("  Loading keywords...");
                Keywords::load(script_location)?
            },
            transforms: {
                println!("  Loading transforms...");
                Transforms::load(script_location)?
            },
            synonyms: {
                println!("  Loading synonyms...");
                Synonyms::load(script_location)?
            },
            reflections: {
                println!("  Loading reflections...");
                Reflections::load(script_location)?
            },
        };

        Ok(e)
    }

    pub fn greet(&self) -> String {
        match self.greetings.random(){
            Some(greet) => greet.to_string(),
            None => String::from("Hello, I am Eliza."), //If greetings are empty, have default
        }
    }

    pub fn farewell(&self) -> String {
        match self.farewells.random(){
            Some(farwell) => farwell.to_string(),
            None => String::from("Goodbye."), //If farewells are empty, have default
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_eliza_okay() {
        assert!(Eliza::new("scripts/rogerian_psychiatrist").is_ok());
    }
}
