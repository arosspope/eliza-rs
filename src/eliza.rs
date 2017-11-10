use std::error::Error;

use script_loader::ScriptLoader;
use reflections::Reflections;
use keywords::{Keywords, Keyword};
use greetings::Greetings;
use farewells::Farewells;
use fallbacks::Fallbacks;
use synonyms::Synonyms;
use transforms::Transforms;

#[derive(Debug, Clone)]
struct PhraseWords {
    phrase : String,      //A phrase
    words : Vec<String>,  //The words that make up the phrase
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
        //Convert the input to lowercase and replace certain words before splitting up the input
        //into phrases and their word parts
        let mut response = String::new();
        let phrases = get_phrases(&self.transform_input(&input.to_lowercase()));

        let (active_phrase, keystack) = self.populate_keystack(phrases);
        //TODO: will need to only create the PhraseWord struct once we have done this.

        if let Some(phrase) = active_phrase {
            //let phrase_to_decompose = phrase_words[pos];
            response = String::from("we got a match");
        } else if false {
            response = String::from("no match");
        } else {
            //Nothing else to try, use fallback trick
            response = self.fallback();
        }

        response
    }

    fn fallback(&self) -> String {
        match self.fallbacks.random(){
            Some(fallback) => fallback.to_string(),
            None => String::from("Go on."), //A fallback for the fallback - har har
        }
    }

    fn transform_input(&self, input: &str) -> String {
        let mut transformed = String::from(input);
        for t in &self.transforms.transforms {
            let replacement = &t.word;
            for equivalent in &t.equivalents {
                transformed = transformed.replace(equivalent, &replacement);
            }
        }

        transformed
    }

    fn populate_keystack(&self, phrases: Vec<String>) -> (Option<String>, Vec<Keyword>)
    {
        let mut keystack: Vec<Keyword> = Vec::new();
        let mut active_phrase: Option<String> = None;

        for phrase in phrases {
            if active_phrase.is_some() {
                //A phrase with keywords was found, break as we don't care about other phrases
                break;
            }

            let words = get_words(&phrase);

            for word in words {
                if let Some(k) = self.kwords.keywords.iter().find(|ref k| k.key == word){
                    keystack.push(k.clone());
                    active_phrase = Some(phrase.clone());
                }
            }
        }

        //sort the keystack with highest rank first
        keystack.sort_by(|a,b| b.rank.cmp(&a.rank));

        (active_phrase, keystack)
    }
}

fn get_phrases(input: &str) -> Vec<String> {
    input.split(|c| c == '.' || c == ',' || c == '?').map(|s| s.to_string()).collect()
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
    fn transform(){
        let e = Eliza::new("scripts/rogerian_psychiatrist").unwrap();

        let t1 = "computers will one day be the superior machine.";
        let t2 = "you're not wrong there.";
        let t3 = "In some ways, you are identical to my father.";
        let t4 = "I can't recollect.";

        assert_eq!("computer will one day be the superior computer.", e.transform_input(t1));
        assert_eq!("you are not wrong there.", e.transform_input(t2));
        assert_eq!("In some ways, you are alike to my father.", e.transform_input(t3));
        assert_eq!("I cant remember.", e.transform_input(t4));
    }

    #[test]
    fn phrase_spliting(){
        let phrases = get_phrases("Hello how are you, you look good. Let    me know what you think,of me?");

        //check phrases are correct
        assert_eq!("Hello how are you", phrases[0]);
        assert_eq!(" you look good", phrases[1]);
        assert_eq!(" Let    me know what you think", phrases[2]);
        assert_eq!("of me", phrases[3]);
    }

    #[test]
    fn word_splitting(){
        let words = get_words("Hello how are you");
        assert_eq!(vec!("Hello", "how", "are", "you"), words);
    }

    #[test]
    fn keystack_building(){
        let e = Eliza::new("scripts/rogerian_psychiatrist").unwrap();

        let phrases = get_phrases("hello how are you? i was feeling good today, but now i'm not.");
        let (phrase, keystack) = e.populate_keystack(phrases);

        assert_eq!("hello how are you", phrase.unwrap());
        assert_eq!(4, keystack.len());
        assert_eq!("hello", keystack[0].key);
        assert_eq!("how", keystack[1].key);
        assert_eq!("are", keystack[2].key);
        assert_eq!("you", keystack[3].key);

        let phrases = get_phrases("spagetti meatballs? i was feeling good today, but now...");
        let (phrase, keystack) = e.populate_keystack(phrases);

        assert_eq!(" i was feeling good today", phrase.unwrap());
        assert_eq!(2, keystack.len());
        assert_eq!("was", keystack[0].key);
        assert_eq!("i", keystack[1].key);

        //check rank ordering
        let phrases = get_phrases("i love my dog - people think we are alike");
        let (phrase, keystack) = e.populate_keystack(phrases);

        assert_eq!("i love my dog - people think we are alike", phrase.unwrap());
        assert_eq!(4, keystack.len());
        assert_eq!("alike", keystack[0].key);
        assert_eq!("my", keystack[1].key);
        assert_eq!("i", keystack[2].key);
        assert_eq!("are", keystack[3].key);
    }

    #[test]
    fn keylist(){
        let e = Eliza::new("scripts/rogerian_psychiatrist").unwrap();
    }
}
