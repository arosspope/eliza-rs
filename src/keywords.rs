use script_loader::ScriptLoader;

#[derive(Serialize, Deserialize, Debug)]
struct Rule {
    decomposition : String,
    reconstruction : Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Keyword {
    key : String,
    rank : u8,
    rules : Vec<Rule>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keywords {
    keywords: Vec<Keyword>,
}

impl ScriptLoader for Keywords {
    type Type = Keywords;

    fn filename() -> String {
        String::from("keywords.json")
    }
}
