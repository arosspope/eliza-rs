extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Rule {
    decomposition : String,
    reconstruction : Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Keyword {
    key : String,
    rank : u8,
    rules : Vec<Rule>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keywords {
    keywords: Vec<Keyword>,
}

impl Keywords {
    pub fn load(s: &str) -> Result<Keywords, Box<Error>> {
        let mut path = PathBuf::from(s);
        path.push("keywords.json");

        //let file = File::open(path.join("keywords.json"))?;
        let file = File::open(path.as_path())?;

        // Read the JSON contents of the file as an instance of `Keywords`.
        let u = serde_json::from_reader(file)?;

        Ok(u)
    }
}
