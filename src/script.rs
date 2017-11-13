extern crate serde;
extern crate serde_json;
extern crate rand;

use self::rand::Rng;
use self::serde::de::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transform {
    pub word: String,
    pub equivalents: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Synonym {
    pub word: String,
    pub equivalents: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reflection {
    pub word: String,
    pub inverse: String,
    pub twoway: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    pub memorise: bool,
    pub decomposition_rule : String,
    pub reassembly_rules : Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keyword {
    pub key : String,
    pub rank : u8,
    pub rules : Vec<Rule>,
}

#[derive(Serialize, Deserialize)]
pub struct Script {
    pub greetings: Vec<String>,
    pub farewells: Vec<String>,
    pub fallbacks: Vec<String>,
    pub transforms: Vec<Transform>,
    pub synonyms: Vec<Synonym>,
    pub reflections: Vec<Reflection>,
    pub keywords: Vec<Keyword>,
}

impl Script {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Script, Box<Error>>
        where for <'de> Script: Deserialize<'de>
    {
        //Attempt to open file and parse the script
        let file = File::open(path)?;
        let script: Script = serde_json::from_reader(file)?;
        Ok(script)
    }

    pub fn rand_greet(&self) -> Option<&String> {
       rand::thread_rng().choose(&self.greetings)
    }

    pub fn rand_farewell(&self) -> Option<&String> {
       rand::thread_rng().choose(&self.farewells)
    }

    pub fn rand_fallback(&self) -> Option<&String> {
       rand::thread_rng().choose(&self.fallbacks)
    }
}
