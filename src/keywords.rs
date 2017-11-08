use script_loader::ScriptLoader;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    pub decomposition : String,
    pub reconstruction : Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keyword {
    pub key : String,
    pub memorise: bool,
    pub rank : u8,
    pub rules : Vec<Rule>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keywords {
    pub keywords: Vec<Keyword>,
}

impl ScriptLoader for Keywords {
    type Type = Keywords;

    fn filename() -> String {
        String::from("keywords.json")
    }
}
