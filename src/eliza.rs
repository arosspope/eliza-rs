use std::error::Error;

use script_loader::ScriptLoader;
use reflections::Reflections;
use keywords::Keywords;
use greetings::Greetings;
use farewells::Farewells;
use fallbacks::Fallbacks;
use synonyms::Synonyms;
use transforms::Transforms;

struct PhraseWords {
    phrase : String,        //A phrase
    words : Vec<String>,      //The words that make up the phrase
}


pub struct Eliza {
    greetings : Greetings,          //A collection of greetings to say 'hello'
    farewells : Farewells,          //A collection of farewells to say 'goodbye'
    fallbacks : Fallbacks,          //A collection of fallback phrases to use, when eliza doesnt know what to do
    kwords : Keywords,              //A collection of keywords and associated decomposition rules
    transforms : Transforms,        //TODO: Things to transform in post processing?
    synonyms : Synonyms,            //TODO: Common synonyms
    reflections : Reflections,      //TODO: Applied before checking composition rules?
    memory : Vec<String>,           //TODO: A collection of things the user has said in previous conversation
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
            fallbacks: {
                println!("  Loading fallbacks...");
                Fallbacks::load(script_location)?
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
            memory: Vec::new(),
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

    pub fn respond(&self, input: &str) -> String {
        let phrases = get_phrases(input);

        self.fallback() //TODO: temporary test code
    }

    fn fallback(&self) -> String {
        match self.fallbacks.random(){
            Some(fallback) => fallback.to_string(),
            None => String::from("Go on."), //A fallback for the fallback - har har
        }
    }
}

fn get_phrases(input: &str) -> Vec<PhraseWords> {
    //TODO: This could probably be done better with lambdas
    let mut phrases: Vec<PhraseWords> = Vec::new();

    for split1 in input.split(","){
        //We also need to split on periods as we are treating them as phrase boundaries
        for split2 in split1.split("."){
            let words = get_words(split2);
            phrases.push(PhraseWords {
                phrase: String::from(split2),
                words });
        }
    }

    phrases
}

fn get_words(phrase: &str) -> Vec<String> {
    phrase.split_whitespace().map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_eliza_okay() {
        assert!(Eliza::new("scripts/rogerian_psychiatrist").is_ok());
    }

    #[test]
    fn phrase_spliting(){
        let phrases = get_phrases("Hello how are you, you look good. Let    me know what you think,of me?");

        //check phrases are correct
        assert_eq!("Hello how are you", phrases[0].phrase);
        assert_eq!(" you look good", phrases[1].phrase);
        assert_eq!(" Let    me know what you think", phrases[2].phrase);
        assert_eq!("of me?", phrases[3].phrase);

        //check words are correct
        assert_eq!(vec!("Hello", "how", "are", "you"), phrases[0].words);
        assert_eq!(vec!("you", "look", "good"), phrases[1].words);
        assert_eq!(vec!("Let", "me", "know", "what", "you", "think"), phrases[2].words);
        assert_eq!(vec!("of", "me?"), phrases[3].words);
    }
}
