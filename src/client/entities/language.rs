
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Language {
    id_language: String,
    language_name: String,
}
