use script_loader::ScriptLoader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transform {
    pub word: String,
    pub equivalents: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transforms {
    pub transforms: Vec<Transform>,
}

impl ScriptLoader for Transforms {
    type Type = Transforms;

    fn filename() -> String {
        String::from("transforms.json")
    }
}
