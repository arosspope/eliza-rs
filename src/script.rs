//! The script defines a set of rules which enable ELIZA to engage in discourse with a user.
//!
//! The beauty of ELIZA's design methodology means that the role of the programmer and playwright
//! are separated. An important property of ELIZA is that a script is data - it is not part of the
//! program itself. Hence, ELIZA is not restricted to a particular set of recognition patterns or
//! responses, indeed not even to any specific language.
//!
//! ## Script Structure
//!
//! The script is written in `json` and is composed of the following.
//!
//! ```json,no_run
//! {
//!     "greetings" : ["", ...],
//!     "farewells" : ["", ...],
//!     "fallbacks" : ["", ...],
//!     "transforms" : [
//!         {"word": "", "equivalents": ["", ...]},
//!         ...
//!     ],
//!     "synonyms" : [
//!         {"word": "", "equivalents": ["", ...]},
//!         ...
//!     ],
//!     "reflections" : [
//!         {"word": "", "inverse": ["", ...], "twoway": bool},
//!         ...
//!     ],
//!     "keywords" : [
//!         {
//!             "key": "", "rank": number,
//!             "rules": [
//!                 {
//!                     "memorise": bool, "decomposition_rule": rust_regex,
//!                     "reassembly_rules": ["", ...]
//!                 },
//!                 ...
//!             ]
//!         },
//!         ...
//!     ]
//! }
//! ```
//!
//! See struct documentation for more information on each element.
//!
use rand;
use serde;
use serde_json;

use rand::seq::SliceRandom;
use self::serde::de::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;

///  A rule to transform a user's input prior to processing.
///
/// # Example
/// For example, if we had the `Transform` rule:
///
/// ```json,no_run
/// { "word" : "remember", "equivalents" : ["recollect", "recall"]}
/// ```
/// Then the text `"I can't recollect, or even recall nowdays"` would be transformed to
/// `"I can't remember, or even remember nowdays"` before performing a keyword search.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Transform {
    pub word: String,
    pub equivalents: Vec<String>,
}

/// A rule to aid the playwright in constructing simple decomposition rules.
///
/// # Example
/// For example, if we had the `Synonym` rule:
///
/// ```json,no_run
/// { "word" : "family", "equivalents" : ["mother","father","sister","brother"]}
/// ```
/// Then the decomposition rule `"(.*)my (.*@family)(.*)"`, would be tried with the following
/// perumtations:
///
/// * `"(.*)my (.*family)(.*)"`
/// * `"(.*)my (.*mother)(.*)"`
/// * `"(.*)my (.*father)(.*)"`
/// * `"(.*)my (.*sister)(.*)"`
/// * `"(.*)my (.*brother)(.*)"`
///
/// Note the special `@` symbol denotes that the word should be permutated.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Synonym {
    pub word: String,
    pub equivalents: Vec<String>,
}

/// A set of string pairs, used to post process any contextual information in an ELIZA
/// response.
///
/// # Example
/// For example, if we had the `Reflection` rules:
///
/// ```json,no_run
/// { "word" : "your", "inverse" : "my", "twoway" : true},
/// { "word" : "i", "inverse" : "you", "twoway" : true}
/// ```
/// * The reassembly rule: `"Really, $2?"`
/// * The contextual information: `$2 = I think about my life`
///
/// Then the assembled response would look like `"Really, you think about your life?"`
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Reflection {
    pub word: String,
    pub inverse: String,
    pub twoway: bool,
}

///  A rule to decompose a user's input then assemble a response based on that input.
///
/// * **memorise**: Used to indicate whether the response should be used now, or saved to
/// internal memory for later use (true).
/// * **decomposition_rule**: A rust regex used to match and extract contextual information from
/// user input.
/// * **reassembly_rules**: A list of strings that are to be used for ELIZA's reponse if the
/// associated `decomposition_rule` matched.
///
/// # Example
/// For example, if we had the `Rule`:
///
/// ```json,no_run
/// { "memorise" : false, "decomposition_rule": "(.*)my(.+)",
///   "reassembly_rules" : ["Really, $2?"]}
/// ```
/// Then the input `"I think about my life"` would match and the assembled response would look like
/// `"Really, life?"`.
///
/// Note the special `$[num]` symbol denotes that a replacement with a regex capture group should
/// occur.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    pub memorise: bool,
    pub decomposition_rule: String,
    pub reassembly_rules: Vec<String>,
}

///  A keyword and it's associated decompositon and reassembly rules.
///
/// * **key**: The keyword to look for in the input text.
/// * **rank**: Denotes it's importance over other keywords. Higher rank = Higher priority.
/// * **rules**: The associated decompositon and reassembly rules
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keyword {
    pub key: String,
    pub rank: u8,
    pub rules: Vec<Rule>,
}

/// A collection of ELIZA directives.
///
/// * **greetings**: A set of strings that are used to greet the user upon program start
/// * **farewells**: A set of strings that are used to farewell the user upon program termination
/// * **fallbacks**: A set of strings that are used when ELIZA can't match any
/// keywords/decompositon rules against user input
/// * **transforms**: A set of rules to transform a user's input prior to processing.
/// * **synonyms**: A set of synonyms to aid the playwright in constructing simple decomposition
/// rules
/// * **reflections**: A set of string pairs, that are used to post process any contextual
/// information in an ELIZA response.
/// * **keywords**: A set of keywords and their associated decompositon and reassembly rules.
///
#[derive(Default, Serialize, Deserialize)]
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
    /// Will load an ELIZA json script from the file system.
    ///
    /// Will return `Err` if the script at the specified location is invalid or non-existant.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Script, Box<dyn Error>>
    where
        for<'de> Script: Deserialize<'de>,
    {
        //Attempt to open file and parse the script
        let file = File::open(path)?;
        let script: Script = serde_json::from_reader(file)?;
        Ok(script)
    }

    pub fn from_str(val: &str) -> Result<Script, Box<dyn Error>> {
        let script: Script = serde_json::from_str(val)?;
        Ok(script)
    }

    /// Returns a random string from the `greetings` vector.
    ///
    /// Will return None if the vector is empty.
    pub fn rand_greet(&self) -> Option<&String> {
        self.greetings.choose(&mut rand::thread_rng())
    }

    /// Returns a random string from the `farewell` vector.
    ///
    /// Will return None if the vector is empty.
    pub fn rand_farewell(&self) -> Option<&String> {
        self.farewells.choose(&mut rand::thread_rng())
        // rand::thread_rng().choose(&self.farewells)
    }

    /// Returns a random string from the `fallback` vector.
    ///
    /// Will return None if the vector is empty.
    pub fn rand_fallback(&self) -> Option<&String> {
        self.fallbacks.choose(&mut rand::thread_rng())
    }
}
