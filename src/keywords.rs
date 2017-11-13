use script_loader::ScriptLoader;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    pub memorise: bool,
    pub decomposition_rule : String,
    pub reassembly_rules : Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keyword {
    pub key : String,
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
