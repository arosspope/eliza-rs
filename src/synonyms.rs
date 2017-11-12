use script_loader::ScriptLoader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Synonym {
    pub word: String,
    pub equivalents: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Synonyms {
    pub synonyms: Vec<Synonym>,
}

impl ScriptLoader for Synonyms {
    type Type = Synonyms;

    fn filename() -> String {
        String::from("synonyms.json")
    }
}
