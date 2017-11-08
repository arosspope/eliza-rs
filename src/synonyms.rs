use script_loader::ScriptLoader;

#[derive(Serialize, Deserialize, Debug)]
struct Synonym {
    word: String,
    equivalents: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Synonyms {
    synonyms: Vec<Synonym>,
}

impl ScriptLoader for Synonyms {
    type Type = Synonyms;

    fn filename() -> String {
        String::from("synonyms.json")
    }
}
