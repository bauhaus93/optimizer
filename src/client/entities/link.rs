
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Link {
    rel: String,
    href: String,
    method: String
}
