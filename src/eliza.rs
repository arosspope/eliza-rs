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
        //Convert the input to lowercase and replace certain words before splitting up the input
        //into phrases and their word parts
        let phrase_words = get_phrase_words(&self.transform_input(&input.to_lowercase()));

        //pass through each phrase -> and use transform.json to swap out words for others
        //TODO: will need to only create the PhraseWord struct once we have done this.

        self.fallback() //TODO: temporary test code
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
}

fn get_phrase_words(input: &str) -> Vec<PhraseWords> {
    let mut phrase_words: Vec<PhraseWords> = Vec::new();

    for p in get_phrases(input) {
        phrase_words.push(PhraseWords {
            phrase: String::from(p),
            words: get_words(p),
        })
    }

    phrase_words
}

fn get_phrases(input: &str) -> Vec<&str> {
    input.split(|c| c == '.' || c == ',').collect()
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
        assert_eq!("of me?", phrases[3]);
    }

    #[test]
    fn phrase_word_spliting(){
        let phrases = get_phrase_words("Hello how are you, you look good. Let    me know what you think,of me?");

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
