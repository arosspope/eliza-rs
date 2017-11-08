extern crate rand;

use self::rand::Rng;
use script_loader::ScriptLoader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Farewells {
    messages: Vec<String>,
}

impl ScriptLoader for Farewells {
    type Type = Farewells;

    fn filename() -> String {
        String::from("farewells.json")
    }
}

impl Farewells {
    pub fn random(&self) -> Option<&String>{
       rand::thread_rng().choose(&self.messages)
   }
}
