extern crate rand;
extern crate serde_json;

use self::rand::Rng;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Messages {
    messages: Vec<String>,
}

impl Messages {
    pub fn load(path: &str, filename: &str) -> Result<Messages, Box<Error>> {
        let path = PathBuf::from(path).join(filename);

        let file = File::open(path.as_path())?;

        // Read the JSON contents of the file as an instance of `Keywords`.
        let u = serde_json::from_reader(file)?;

        Ok(u)
    }

    pub fn random(&self) -> Option<&String>{
        rand::thread_rng().choose(&self.messages)
    }
}
