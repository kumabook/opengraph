#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Audio {
    #[serde(rename = "type")]
    pub obj_type:   Option<String>,
    pub url:        String,
    pub secure_url: Option<String>,
}

impl Audio {
    pub fn new(url: String) -> Audio {
        Audio { url: url, secure_url: None, obj_type: None }
    }
}
