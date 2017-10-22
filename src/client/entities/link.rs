
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    rel: String,
    href: String,
    method: String
}
