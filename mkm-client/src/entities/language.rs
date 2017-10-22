#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    id_language: u32,
    language_name: String,
}
