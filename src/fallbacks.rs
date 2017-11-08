extern crate rand;

use self::rand::Rng;
use script_loader::ScriptLoader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Fallbacks {
    messages: Vec<String>,
}

impl ScriptLoader for Fallbacks {
    type Type = Fallbacks;

    fn filename() -> String {
        String::from("fallbacks.json")
    }
}

impl Fallbacks {
    pub fn random(&self) -> Option<&String>{
       rand::thread_rng().choose(&self.messages)
   }
}
