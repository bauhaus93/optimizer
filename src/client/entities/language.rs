use serde_json;

#[derive(Deserialize, Debug, Clone)]
pub struct Language {
    #[serde(rename="idLanguage")]
    language_id: String,
    #[serde(rename="languageName")]
    language_name: String,
}
