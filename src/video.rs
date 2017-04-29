#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Video {
    #[serde(rename = "type")]
    pub obj_type:   Option<String>,
    pub url:        String,
    pub secure_url: Option<String>,
    pub width:      Option<i32>,
    pub height:     Option<i32>
}

impl Video {
    pub fn new(url: String) -> Video {
        Video {
            url:        url,
            secure_url: None,
            obj_type:   None,
            width:      None,
            height:     None,
        }
    }
}
