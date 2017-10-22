#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    id_language: String,
    language_name: String,
}
