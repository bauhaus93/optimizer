#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    id_category: u32,
    category_name: String,
}
