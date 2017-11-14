#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate regex;

mod alphabet;
mod script;

use std::error::Error;
use std::collections::{VecDeque, HashMap};
use regex::{Regex, Captures};
use alphabet::Alphabet;
use script::{Script, Keyword, Reflection, Synonym, Transform};

pub struct Eliza {
    script : Script,
    memory : VecDeque<String>,
    rule_usage : HashMap<String, usize>,
}

impl Eliza {
    pub fn new(location: &str) -> Result<Eliza, Box<Error>> {
        let e = Eliza {
            script: {
                info!("Loading script {}", location);
                Script::load(location)?
            },
            memory: VecDeque::new(),
            rule_usage: HashMap::new(),
        };

        Ok(e)
    }

    pub fn greet(&self) -> String {
        match self.script.rand_greet(){
            Some(greet) => greet.to_string(),
            None => {
                warn!("Eliza has no greetings to use");
                String::from("Hello, I am Eliza.") //If greetings are empty, have default
            }
        }
    }

    pub fn farewell(&self) -> String {
        match self.script.rand_farewell(){
            Some(farwell) => farwell.to_string(),
            None => {
                warn!("Eliza has no farewells to use");
                String::from("Goodbye.") //If farewells are empty, have default
            }
        }
    }

    pub fn respond(&mut self, input: &str) -> String {
        //Convert the input to lowercase and transform words before populating the keystack
        let mut response: Option<String> = None;
        let phrases = get_phrases(&transform(&input.to_lowercase(), &self.script.transforms));
        let (active_phrase, mut keystack) = populate_keystack(phrases, &self.script.keywords);

        if let Some(phrase) = active_phrase {
            response = self.get_response(&phrase, &mut keystack);
        }

        if let Some(res) = response {
            res
        } else if let Some(mem) = self.memory.pop_front(){
            //Attempt to use something in memory, otherwise use fallback trick
            info!("Using memory");
            mem
        } else {
            info!("Using fallback statement");
            self.fallback()
        }
    }

    fn fallback(&self) -> String {
        match self.script.rand_fallback() {
            Some(fallback) => fallback.to_string(),
            None => {
                warn!("Eliza has no fallbacks to use");
                String::from("Go on.") //A fallback for the fallback - har har
            }
        }
    }

    fn get_response(&mut self, phrase: &str, keystack: &mut VecDeque<Keyword>) -> Option<String> {
        let mut response: Option<String> = None;

        //Search for a response while the keystack is not empty
        'search: while response.is_none() && !keystack.is_empty(){
            let next = keystack.pop_front().unwrap(); //safe due to prior check

            //For each rule set, attempt to decompose phrase then reassemble a response
            'decompostion: for r in next.rules {
                //Get all regex permutations of the decomposition rule (dependent upon synonyms)
                let regexes = permutations(&r.decomposition_rule, &self.script.synonyms);
                for re in regexes {
                    if let Some(cap) = re.captures(phrase)
                    {
                        //A match was found: find the best reassembly rule to use
                        if let Some(assem) = self.get_reassembly(&r.reassembly_rules)
                        {
                            if let Some(goto) = is_goto(&assem) {
                                //The best rule was a goto, push associated key entry to stack
                                if let Some(entry) =
                                    self.script.keywords.iter().find(|ref a| a.key == goto)
                                {
                                    //Push to front of keystack and skip to it
                                    keystack.push_front(entry.clone());
                                    break 'decompostion;
                                } else {
                                    error!("No such keyword: {}", goto);
                                    continue; //Something wrong with this GOTO
                                }
                            }

                            //Attempt to assemble given the capture groups
                            response = assemble(&assem, &cap, &self.script.reflections);
                            if response.is_some(){
                                if r.memorise {
                                    //We'll save this response for later...
                                    self.memory.push_back(response.unwrap());
                                    response = None;
                                } else {
                                    //We found a response, exit
                                    break 'search;
                                }
                            }
                        }
                    }
                }
            }
        }

        response
    }

    fn get_reassembly(&mut self, rules: &[String]) -> Option<String> {
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
}

fn transform(input: &str, transforms: &[Transform]) -> String {
    let mut transformed = String::from(input);
    for t in transforms {
        let replacement = &t.word;
        for equivalent in &t.equivalents {
            transformed = transformed.replace(equivalent, &replacement);
        }
    }

    transformed
}

fn populate_keystack(phrases: Vec<String>, keywords: &[Keyword])
    -> (Option<String>, VecDeque<Keyword>)
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
            if let Some(k) = keywords.iter().find(|ref k| k.key == word){
                keystack.push(k.clone());
                active_phrase = Some(phrase.clone());
            }
        }
    }

    //sort the keystack with highest rank first
    keystack.sort_by(|a,b| b.rank.cmp(&a.rank));

    (active_phrase, VecDeque::from(keystack))
}

fn permutations(decomposition: &str, synonyms: &[Synonym]) -> Vec<Regex> {
    let mut permutations: Vec<String> = Vec::new();
    let mut re_perms: Vec<Regex> = Vec::new();
    let words = get_words(decomposition);

    if decomposition.matches('@').count() > 1 {
        error!("Decomposition rules are limited to one synonym conversion: '{}'", decomposition);
        return re_perms;
    }

    //If no '@' symbol then just add to permutations
    if decomposition.matches('@').count() == 0 {
        permutations.push(decomposition.to_string());
    } else {
        //remember to add the base word without the @
        permutations.push(decomposition.replace('@', ""));
    }

    for w in &words {
        if w.contains("@"){
            //Format example: '(.*) my (.* @family)'
            let scrubbed = alphabet::STANDARD.scrub(w);
            if let Some(synonym) = synonyms.iter().find(|ref s| s.word == scrubbed) {
                for equivalent in &synonym.equivalents {
                    permutations.push(decomposition.replace(&scrubbed, &equivalent).replace('@', ""));
                }
            }
        }
    }

    for p in permutations {
        if let Ok(re) = Regex::new(&p) {
            re_perms.push(re)
        } else {
            error!("Invalid decompostion rule: '{}'", decomposition);
        }
    }

    re_perms
}

fn assemble(rule: &str, captures: &Captures, reflections: &[Reflection]) -> Option<String>
{
    //TODO: Better way using regex replace all?
    let mut temp = String::from(rule);
    let mut ok = true;
    let words = get_words(rule);

    //For each word, see if we need to swap anything out for a capture
    for w in &words {
        if w.contains("$"){
            //Format example 'What makes you think I am $2 ?' which
            //uses the second capture group of the regex
            let scrubbed = alphabet::ALPHANUMERIC.scrub(w);
            if let Ok(n) = scrubbed.parse::<usize>() {
                if n < captures.len() + 1 { //indexing starts at 1
                    //Perform reflection on the capture before subsitution
                    temp = temp.replace(&scrubbed, &reflect(&captures[n], reflections)).replace("$", "");
                } else {
                    ok = false;
                    error!("{} is outside capture range in: '{}'", n, rule);
                }
            } else {
                ok = false;
                error!("Contains invalid capture id: '{}'", rule);
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
                error!("Invalid reflection for pair {:?} in: '{}'", reflect, input);
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
    input.split(|c| c == '.' || c == ',' || c == '?').map(|s| s.trim().to_string()).collect()
}

fn get_words(phrase: &str) -> Vec<String> {
    phrase.split_whitespace().map(|s| s.to_string()).collect()
}

//Returns NONE is not goto, otherwise reutrns goto statement
fn is_goto(statement: &str) -> Option<String> {
    match statement.contains("GOTO"){
        true => Some(statement.replace("GOTO", "").replace(char::is_whitespace, "")),
        false => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use script::{Rule};

    #[test]
    fn perm_valid(){
        let synonyms: Vec<Synonym> = vec!(Synonym {
            word: "family".to_string(),
            equivalents: vec!("brother".to_string(), "mother".to_string())
        });

        let re_perms = permutations("(.*)my (.* @family)", &synonyms);
        assert_eq!("(.*)my (.* family)", re_perms[0].as_str());
        assert_eq!("(.*)my (.* brother)", re_perms[1].as_str());
        assert_eq!("(.*)my (.* mother)", re_perms[2].as_str());
    }

    #[test]
    fn perm_invalid(){
        let synonyms: Vec<Synonym> = vec!(Synonym {
            word: "family".to_string(),
            equivalents: vec!("brother".to_string(), "mother".to_string())
        });

        let re_perms = permutations("(.*)my (.* @family @fail)", &synonyms);
        assert!(re_perms.is_empty());
    }

    #[test]
    fn perm_simple(){
        let synonyms: Vec<Synonym> = vec!(Synonym {
            word: "family".to_string(),
            equivalents: vec!("brother".to_string(), "mother".to_string())
        });

        let re_perms = permutations("(.*)my (.* dog)", &synonyms);
        assert_eq!(1, re_perms.len());
        assert_eq!("(.*)my (.* dog)", re_perms[0].as_str());
    }

    #[test]
    fn assemble_rule_equal(){
        let mut e = Eliza::new("scripts/rogerian_psychiatrist.json").unwrap();

        //Create a fake rule usage HashMap
        let mut usages: HashMap<String, usize> = HashMap::new();
        usages.insert("first".to_string(), 1);
        usages.insert("second".to_string(), 1);
        usages.insert("third".to_string(), 1);
        usages.insert("fourth".to_string(), 1);

        //All equal precedence, should just return the first
        e.rule_usage = usages;
        assert_eq!("first", e.get_reassembly(&vec!("first".to_string(), "second".to_string(), "third".to_string(), "fourth".to_string())).unwrap());
        assert_eq!(2, e.rule_usage["first"]);
    }

    #[test]
    fn assemble_rule_smaller(){
        let mut e = Eliza::new("scripts/rogerian_psychiatrist.json").unwrap();

        //Create a fake rule usage HashMap
        let mut usages: HashMap<String, usize> = HashMap::new();
        usages.insert("first".to_string(), 7);
        usages.insert("second".to_string(), 3);
        usages.insert("third".to_string(), 2);
        usages.insert("fourth".to_string(), 10);

        //One has been used less than the rest
        e.rule_usage = usages;
        assert_eq!("third", e.get_reassembly(&vec!("first".to_string(), "second".to_string(), "third".to_string(), "fourth".to_string())).unwrap());
        assert_eq!(3, e.rule_usage["third"]);
    }

    #[test]
    fn assemble_rule_unknown(){
        let mut e = Eliza::new("scripts/rogerian_psychiatrist.json").unwrap();

        //Create a fake rule usage HashMap
        let mut usages: HashMap<String, usize> = HashMap::new();
        usages.insert("first".to_string(), 7);
        usages.insert("second".to_string(), 3);
        usages.insert("third".to_string(), 2);

        //One has never been used
        e.rule_usage = usages;
        assert_eq!("fourth", e.get_reassembly(&vec!("first".to_string(), "second".to_string(), "third".to_string(), "fourth".to_string())).unwrap());
        assert_eq!(1, e.rule_usage["fourth"]);
    }

    #[test]
    fn assemble_ok(){
        let reflections: Vec<Reflection> = Vec::new();
        let re = Regex::new(r"(.*) you are (.*)").unwrap();
        let phrase = "I think that you are so stupid";
        let cap = re.captures(phrase).unwrap();

        let res = assemble("What makes you think I am $2?", &cap, &reflections);
        assert_eq!(res.unwrap(), "What makes you think I am so stupid?");
    }

    #[test]
    fn assemble_invalid_index(){
        let reflections: Vec<Reflection> = Vec::new();
        let re = Regex::new(r"(.*) you are (.*)").unwrap();
        let phrase = "I think that you are so stupid";
        let cap = re.captures(phrase).unwrap();

        let res = assemble("What makes you think I am $5 ?", &cap, &reflections);
        assert!(res.is_none());
    }

    #[test]
    fn assemble_invalid_id(){
        let reflections: Vec<Reflection> = Vec::new();
        let re = Regex::new(r"(.*) you are (.*)").unwrap();
        let phrase = "I think that you are so stupid";
        let cap = re.captures(phrase).unwrap();

        let res = assemble("What makes you think I am $a ?", &cap, &reflections);
        assert!(res.is_none());
    }

    #[test]
    fn transform_phrases(){
        let transforms = vec!(
            Transform {word: String::from("computer"), equivalents: vec!(String::from("machine"), String::from("computers"))},
            Transform {word: String::from("remember"), equivalents: vec!(String::from("recollect"))}
        );

        assert_eq!("computer will one day be the superior computer.",
            transform("computers will one day be the superior machine.", &transforms));

        assert_eq!("I cant remember.",
            transform("I cant recollect.", &transforms));
    }

    #[test]
    fn keystack_simple(){
        let keywords: Vec<Keyword> = vec!(
            Keyword { key: String::from("hello"), rank: 0, rules: vec!(
                Rule {memorise: false, decomposition_rule: String::new(), reassembly_rules: Vec::new()})},
            Keyword { key: String::from("how"), rank: 0, rules: vec!(
                Rule {memorise: false, decomposition_rule: String::new(), reassembly_rules: Vec::new()})},
            Keyword { key: String::from("i"), rank: 0, rules: vec!(
                Rule {memorise: false, decomposition_rule: String::new(), reassembly_rules: Vec::new()})},
        );

        let phrases = get_phrases("hello how are you? i was feeling good today, but now i'm not.");
        let (phrase, keystack) = populate_keystack(phrases, &keywords);

        assert_eq!("hello how are you", phrase.unwrap());
        assert_eq!(2, keystack.len());
        assert_eq!("hello", keystack[0].key);
        assert_eq!("how", keystack[1].key);
    }

    #[test]
    fn keystack_phrase_select() {
        let keywords: Vec<Keyword> = vec!(
            Keyword { key: String::from("was"), rank: 0, rules: vec!(
                Rule {memorise: false, decomposition_rule: String::new(), reassembly_rules: Vec::new()})},
            Keyword { key: String::from("how"), rank: 0, rules: vec!(
                Rule {memorise: false, decomposition_rule: String::new(), reassembly_rules: Vec::new()})},
            Keyword { key: String::from("i"), rank: 0, rules: vec!(
                Rule {memorise: false, decomposition_rule: String::new(), reassembly_rules: Vec::new()})},
        );

        let phrases = get_phrases("spagetti meatballs? i was feeling good today, but now...");
        let (phrase, keystack) = populate_keystack(phrases, &keywords);

        assert_eq!("i was feeling good today", phrase.unwrap());
        assert_eq!(2, keystack.len());
        assert_eq!("i", keystack[0].key);
        assert_eq!("was", keystack[1].key);
    }

    #[test]
    fn keystack_rank_order() {
        let keywords: Vec<Keyword> = vec!(
            Keyword { key: String::from("i"), rank: 1, rules: vec!(
                Rule {memorise: false, decomposition_rule: String::new(), reassembly_rules: Vec::new()})},
            Keyword { key: String::from("my"), rank: 2, rules: vec!(
                Rule {memorise: false, decomposition_rule: String::new(), reassembly_rules: Vec::new()})},
            Keyword { key: String::from("are"), rank: 0, rules: vec!(
                Rule {memorise: false, decomposition_rule: String::new(), reassembly_rules: Vec::new()})},
            Keyword { key: String::from("alike"), rank: 3, rules: vec!(
                Rule {memorise: false, decomposition_rule: String::new(), reassembly_rules: Vec::new()})},
        );

        let phrases = get_phrases("i love my dog - people think we are alike");
        let (phrase, keystack) = populate_keystack(phrases, &keywords);

        assert_eq!("i love my dog - people think we are alike", phrase.unwrap());
        assert_eq!(4, keystack.len());
        assert_eq!("alike", keystack[0].key);
        assert_eq!("my", keystack[1].key);
        assert_eq!("i", keystack[2].key);
        assert_eq!("are", keystack[3].key);
    }

    #[test]
    fn phrase_spliting(){
        let phrases = get_phrases("Hello how are you, you look good. Let    me know what you think,of me?");

        //check phrases are correct
        assert_eq!("Hello how are you", phrases[0]);
        assert_eq!("you look good", phrases[1]);
        assert_eq!("Let    me know what you think", phrases[2]);
        assert_eq!("of me", phrases[3]);
    }

    #[test]
    fn word_splitting(){
        let words = get_words("Hello how are you");
        assert_eq!(vec!("Hello", "how", "are", "you"), words);
    }
}
