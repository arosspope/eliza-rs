use script_loader::ScriptLoader;

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

impl ScriptLoader for Reflections {
    type Type = Reflections;

    fn filename() -> String {
        String::from("reflections.json")
    }
}
