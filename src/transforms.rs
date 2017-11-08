use script_loader::ScriptLoader;

#[derive(Serialize, Deserialize, Debug)]
struct Transform {
    word: String,
    equivalents: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transforms {
    transforms: Vec<Transform>,
}

impl ScriptLoader for Transforms {
    type Type = Transforms;

    fn filename() -> String {
        String::from("transforms.json")
    }
}
