use reqwest::Url;
use reqwest::UrlError;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Image {
    #[serde(rename = "type")]
    pub obj_type:   Option<String>,
    pub url:        String,
    pub secure_url: Option<String>,
    pub width:      Option<i32>,
    pub height:     Option<i32>
}

impl Image {
    pub fn new(url: String) -> Image {
        Image {
            url:        url,
            secure_url: None,
            obj_type:   None,
            width:      None,
            height:     None,
        }
    }
    pub fn normalize(&mut self, url: &Url) -> &mut Image {
        if let Err(e) = Url::parse(&self.url) {
            match e {
                UrlError::RelativeUrlWithoutBase => {
                    if let Ok(url) = url.join(&self.url) {
                        self.url = url.to_string();
                    }
                },
                _ => (),
            }
            println!("{:?}", e);
        }
        self
    }
}
