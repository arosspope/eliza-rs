extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Reflection {
    word: String,
    inverse: String,
    twoway: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reflections {
    reflections: Vec<Reflection>,
}

impl Reflections {
    pub fn load(path: &str) -> Result<Reflections, Box<Error>> {
        let path = PathBuf::from(path).join("reflections.json");

        let file = File::open(path.as_path())?;
        let u = serde_json::from_reader(file)?;

        Ok(u)
    }
}
