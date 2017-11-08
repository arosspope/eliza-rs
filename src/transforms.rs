extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Transform {
    word: String,
    equivalents: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transforms {
    transforms: Vec<Transform>,
}

impl Transforms {
    pub fn load(path: &str) -> Result<Transforms, Box<Error>> {
        let path = PathBuf::from(path).join("transforms.json");

        let file = File::open(path.as_path())?;
        let u = serde_json::from_reader(file)?;

        Ok(u)
    }
}
