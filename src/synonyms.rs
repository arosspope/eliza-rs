extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Synonym {
    word: String,
    equivalents: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Synonyms {
    synonyms: Vec<Synonym>,
}

impl Synonyms {
    pub fn load(path: &str) -> Result<Synonyms, Box<Error>> {
        let path = PathBuf::from(path).join("synonyms.json");

        let file = File::open(path.as_path())?;
        let u = serde_json::from_reader(file)?;

        Ok(u)
    }
}
