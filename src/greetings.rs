extern crate rand;

use self::rand::Rng;
use script_loader::ScriptLoader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Greetings {
    messages: Vec<String>,
}

impl ScriptLoader for Greetings {
    type Type = Greetings;

    fn filename() -> String {
        String::from("greetings.json")
    }
}

impl Greetings {
    pub fn random(&self) -> Option<&String>{
       rand::thread_rng().choose(&self.messages)
   }
}
