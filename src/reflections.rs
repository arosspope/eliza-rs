use script_loader::ScriptLoader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Reflection {
    pub word: String,
    pub inverse: String,
    pub twoway: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reflections {
    pub reflections: Vec<Reflection>,
}

impl ScriptLoader for Reflections {
    type Type = Reflections;

    fn filename() -> String {
        String::from("reflections.json")
    }
}
