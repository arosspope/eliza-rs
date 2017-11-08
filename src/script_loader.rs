extern crate serde;
extern crate serde_json;

use self::serde::de::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub trait ScriptLoader {
    type Type;

    fn load(path: &str) -> Result<Self::Type, Box<Error>>
        where for <'de> Self::Type: Deserialize<'de>
    {
        let path = PathBuf::from(path).join(Self::filename());

        let file = File::open(path.as_path())?;
        let u: Self::Type = serde_json::from_reader(file)?;

        Ok(u)
    }

    fn filename() -> String;
}
