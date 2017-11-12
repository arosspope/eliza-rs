use std::error::Error;
use std::collections::{VecDeque, HashMap};
use regex::{Regex, Captures};

use script_loader::ScriptLoader;
use reflections::{Reflections, Reflection};
use keywords::{Keywords, Keyword};
use greetings::Greetings;
use farewells::Farewells;
use fallbacks::Fallbacks;
use synonyms::{Synonyms, Synonym};
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
    memory : VecDeque<String>,      //TODO: A collection of things the user has said in previous conversation
    rule_usage : HashMap<String, usize>,
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
            memory: VecDeque::new(),
            rule_usage: HashMap::new(),
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

    fn fallback(&self) -> String {
        match self.fallbacks.random(){
            Some(fallback) => fallback.to_string(),
            None => String::from("Go on."), //A fallback for the fallback - har har
        }
    }

    pub fn respond(&mut self, input: &str) -> String {
        //Convert the input to lowercase and replace certain words before splitting up the input
        //into phrases and their word parts
        let phrases = get_phrases(&self.transform_input(&input.to_lowercase()));

        let (active_phrase, keystack) = self.populate_keystack(phrases);

        let mut response: Option<String> = None;

        if let Some(phrase) = active_phrase {
            response = self.find_response(&phrase, keystack);
        }

        if let Some(res) = response {
            res
        } else if let Some(mem) = self.memory.pop_front(){
            //Attempt to use something in memory, otherwise use fallback trick
            mem
        } else {
            self.fallback()
        }
    }

    fn find_response(&mut self, phrase: &str, keystack: Vec<Keyword>) -> Option<String> {
        let mut response = None;

        'outer: for k in keystack {
            for rule in k.rules {
                //TODO: Static lazy loading??

                //TODO: get a permutation of the regex with @sybmbols
                //Get permutations of the decomposition rule (rules with the '@' symbol)
                let re_perms = rule_permutations(&rule.decomposition, &self.synonyms.synonyms);

                for re in re_perms {
                    if let Some(cap) = re.captures(phrase) {
                        //A match was found: find the best reconstruction rule
                        if let Some(recon) = self.get_recon_rule(&rule.reconstruction) {
                            //TODO: if best rule contained 'GOTO' do that...
                            
                            response = reconstruct(&recon, &cap, &self.reflections.reflections);

                            if response.is_some(){
                                if rule.memorise {
                                    //We'll save this response for later...
                                    self.memory.push_back(response.unwrap());
                                    response = None;
                                } else {
                                    //We found a response, exit
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
        }

        //for each decompostion rule attempt to match
            //We found a match
            //For each recomoposition rule
                //If not in hashmap, add and set number to 1
                //Otherwise loop through each rule and find the smallest one
                //when found, increment its use count and add to self
            //Parse rule and replace '(2)' with the matching group


        //TODO: Swap synonyms when '@' symbol is encountered
        //TODO: Store to memory when 'memorise' is true

        response
    }

    fn get_recon_rule(&mut self, rules: &[String]) -> Option<String> {
        let mut best_rule: Option<String> = None;
        let mut count: Option<usize> = None;

        for rule in rules {
            match self.rule_usage.contains_key(rule) {
                true => {
                    //If it has already been used, get its usage count
                    let usage = self.rule_usage[rule];
                    if let Some(c) = count {
                        if usage < c {
                            //The usage is less than the running total
                            best_rule = Some(rule.clone());
                            count = Some(usage);
                        }
                    } else {
                        //The count has yet to be updated, this is the best usage so far
                        best_rule = Some(rule.clone());
                        count = Some(usage);
                    }
                },
                false => {
                    //The rule has never been used before - this has precedence
                    best_rule = Some(rule.clone());
                    self.rule_usage.insert(rule.to_string(), 0);
                    break;
                }
            }
        }

        //For whatever rule we use (if any), increment its usage count
        if best_rule.is_some(){
            if let Some(usage) = self.rule_usage.get_mut(&best_rule.clone().unwrap()){
                *usage = *usage + 1;
            }
        }

        best_rule
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

fn rule_permutations(rule: &str, synonyms: &[Synonym]) -> Vec<Regex> {
    let mut permutations: Vec<Regex> = Vec::new();

    if let Ok(re) = Regex::new(rule) {
        permutations.push(re)
    } else {
        eprintln!("[ERR] Invalid decompostion rule: '{}'", rule);
    }

    permutations
}

fn reconstruct(rule: &str, captures: &Captures, reflections: &[Reflection]) -> Option<String>
{
    //TODO: Better way using regex replace all?
    //TODO: Must include note about being whitespace before '?'
    let mut temp = String::from(rule);
    let mut ok = true;
    let words = get_words(rule);

    //For each word, see if we need to swap anything out for a capture
    for w in &words {
        if w.contains("$"){
            //Format example 'What makes you think I am $2 ?' which
            //uses the second capture group of the regex
            if let Ok(n) = w.replace("$", "").parse::<usize>() {
                if n < captures.len() + 1 { //indexing starts at 1
                    //Perform reflection on the capture before subsitution
                    temp = temp.replace(w, &reflect(&captures[n], reflections));
                } else {
                    ok = false;
                    eprintln!("[ERR] {} is outside capture range in: '{}'", n, rule);
                }
            } else {
                ok = false;
                eprintln!("[ERR] Contains invalid capture id: '{}'", rule);
            }
        }

        if !ok {
            break;
        }
    }

    if ok {
        Some(temp)
    } else {
        None
    }
}

fn reflect(input: &str, reflections: &[Reflection]) -> String {
    //we don't want to accidently re-reflect word pairs that have two-way reflection
    let mut reflected_phrase = String::new();
    let words = get_words(input);

    for w in words {
        //Find reflection pairs that are applicable to this word
        if let Some(reflect) = reflections.iter().find(|ref r| {
            r.word == w || return if r.twoway {r.inverse == w} else {false}
        }) {
            if reflect.word == w {
                reflected_phrase.push_str(&reflect.inverse);
            } else if reflect.twoway && reflect.inverse == w {
                reflected_phrase.push_str(&reflect.word);
            } else {
                //Unlikely to happen, but print message just incase
                eprintln!("[ERR] Invalid reflection for pair {:?} in: {}", reflect, input);
            }
        } else {
            //No reflection required
            reflected_phrase.push_str(&w);
        }

        reflected_phrase.push_str(" "); //put a space after each word
    }

    reflected_phrase.trim().to_string()
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

    //TODO: Decouple tests from input json files

    #[test]
    fn loading_eliza_okay() {
        assert!(Eliza::new("scripts/rogerian_psychiatrist").is_ok());
    }

    #[test]
    fn regex_test(){
        let re = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
        let before = "2012-03-14, 2013-01-01 and 2014-07-05";
        let after = re.replace_all(before, "$m/$d/$y");
        assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");


        let re = Regex::new(r"(.*) you are (.*)").unwrap();
        let phrase = "I think";
        let cap = re.captures(phrase);
        assert!(cap.is_none());
    }

    #[test]
    fn recon_rule_equal(){
        let mut e = Eliza::new("scripts/rogerian_psychiatrist").unwrap();

        //Create a fake rule usage HashMap
        let mut usages: HashMap<String, usize> = HashMap::new();
        usages.insert("first".to_string(), 1);
        usages.insert("second".to_string(), 1);
        usages.insert("third".to_string(), 1);
        usages.insert("fourth".to_string(), 1);

        //All equal precedence, should just return the first
        e.rule_usage = usages;
        assert_eq!("first", e.get_recon_rule(&vec!("first".to_string(), "second".to_string(), "third".to_string(), "fourth".to_string())).unwrap());
        assert_eq!(2, e.rule_usage["first"]);
    }

    #[test]
    fn recon_rule_smaller(){
        let mut e = Eliza::new("scripts/rogerian_psychiatrist").unwrap();

        //Create a fake rule usage HashMap
        let mut usages: HashMap<String, usize> = HashMap::new();
        usages.insert("first".to_string(), 7);
        usages.insert("second".to_string(), 3);
        usages.insert("third".to_string(), 2);
        usages.insert("fourth".to_string(), 10);

        //One has been used less than the rest
        e.rule_usage = usages;
        assert_eq!("third", e.get_recon_rule(&vec!("first".to_string(), "second".to_string(), "third".to_string(), "fourth".to_string())).unwrap());
        assert_eq!(3, e.rule_usage["third"]);
    }

    #[test]
    fn recon_rule_unknown(){
        let mut e = Eliza::new("scripts/rogerian_psychiatrist").unwrap();

        //Create a fake rule usage HashMap
        let mut usages: HashMap<String, usize> = HashMap::new();
        usages.insert("first".to_string(), 7);
        usages.insert("second".to_string(), 3);
        usages.insert("third".to_string(), 2);

        //One has never been used
        e.rule_usage = usages;
        assert_eq!("fourth", e.get_recon_rule(&vec!("first".to_string(), "second".to_string(), "third".to_string(), "fourth".to_string())).unwrap());
        assert_eq!(1, e.rule_usage["fourth"]);
    }

    #[test]
    fn recon_ok(){
        let reflections: Vec<Reflection> = Vec::new();
        let re = Regex::new(r"(.*) you are (.*)").unwrap();
        let phrase = "I think that you are so stupid";
        let cap = re.captures(phrase).unwrap();

        let res = reconstruct("What makes you think I am $2 ?", &cap, &reflections);
        assert_eq!(res.unwrap(), "What makes you think I am so stupid ?");
    }

    #[test]
    fn recon_invalid_index(){
        let reflections: Vec<Reflection> = Vec::new();
        let re = Regex::new(r"(.*) you are (.*)").unwrap();
        let phrase = "I think that you are so stupid";
        let cap = re.captures(phrase).unwrap();

        let res = reconstruct("What makes you think I am $5 ?", &cap, &reflections);
        assert!(res.is_none());
    }

    #[test]
    fn recon_invalid_id(){
        let reflections: Vec<Reflection> = Vec::new();
        let re = Regex::new(r"(.*) you are (.*)").unwrap();
        let phrase = "I think that you are so stupid";
        let cap = re.captures(phrase).unwrap();

        let res = reconstruct("What makes you think I am $a ?", &cap, &reflections);
        assert!(res.is_none());
    }

    //TODO: Move transform outside of ELIZA for testing
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

    //TODO: Move keystack outside of scope of eliza for testing
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
