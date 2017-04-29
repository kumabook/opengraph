use video::Video;
use image::Image;
use audio::Audio;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Object {
    #[serde(rename = "type")]
    pub obj_type:         ObjectType,
    pub title:            String,
    pub url:              String,

    pub images:           Vec<Image>,
    pub audios:           Vec<Audio>,
    pub videos:           Vec<Video>,

    pub description:      Option<String>,
    pub determiner:       Option<Determiner>,
    pub locale:           Option<String>,
    pub locale_alternate: Vec<String>,
    pub site_name:        Option<String>,
}

impl Object {
    pub fn new<'a>(props: &'a Vec<(String, String)>) -> Object {
        let mut obj = Object::default();
        for prop in props.iter() {
            let key: &str = &(prop.0);
            let v         = prop.1.clone();
            match key {
                "title"       => { obj.title       = v; },
                "type"        => { obj.obj_type    = ObjectType::new(v); },
                "url"         => { obj.url         = v; },
                "description" => { obj.description = Some(v); },
                "determiner"  => { obj.determiner  = Some(Determiner::new(v)); },
                "locale"      => { obj.locale      = Some(v); },
                "site_name"   => { obj.site_name   = Some(v); },

                "image"            => { obj.images.push(Image::new(v)); },
                "video"            => { obj.videos.push(Video::new(v)); },
                "audio"            => { obj.audios.push(Audio::new(v)); },
                "locale:alternate" => {
                    obj.locale_alternate.push(v)
                },
                v if v.starts_with("image") => {
                },
                v if v.starts_with("music") => {
                },

                v if v.starts_with("video") => {
                },
                v if v.starts_with("article") => {
                },
                v if v.starts_with("book") => {
                },
                v if v.starts_with("profile") => {
                },
                _ => {},
            }
        }
        obj
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ObjectType {
    // No Vetical
    #[serde(rename = "article")]
    Article,
    #[serde(rename = "book")]
    Book,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "website")]
    Website,
    // Music
    #[serde(rename = "music.song")]
    Song,
    #[serde(rename = "music.album")]
    Album,
    #[serde(rename = "music.playlist")]
    Playlist,
    #[serde(rename = "music.radio_station")]
    RadioStation,
    // Video
    #[serde(rename = "video.movie")]
    Movie,
    #[serde(rename = "video.episode")]
    Episode,
    #[serde(rename = "video.tv_show")]
    TVShow,
    #[serde(rename = "video.other")]
    VideoOther,
}

impl ObjectType {
    pub fn new(str: String) -> ObjectType {
        match str.as_ref() {
            "article"             => ObjectType::Article,
            "book"                => ObjectType::Book,
            "profile"             => ObjectType::Profile,
            "website"             => ObjectType::Website,
            "music.song"          => ObjectType::Song,
            "music.album"         => ObjectType::Album,
            "music.playlist"      => ObjectType::Playlist,
            "music.radio_station" => ObjectType::RadioStation,
            "video.movie"         => ObjectType::Movie,
            "video.episode"       => ObjectType::Episode,
            "video.tv_show"       => ObjectType::TVShow,
            "video.other"         => ObjectType::VideoOther,
            _                     => ObjectType::Website,
        }
    }
}

impl Default for ObjectType {
    fn default() -> ObjectType  { ObjectType::Website }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Determiner {
    #[serde(rename = "a")]
    A,
    #[serde(rename = "an")]
    An,
    #[serde(rename = "the")]
    The,
    #[serde(rename = "")]
    Blank,
    #[serde(rename = "auto")]
    Auto,
}

impl Determiner {
    pub fn new(str: String) -> Determiner {
        match str.as_ref() {
            "a"     => Determiner::A,
            "an"    => Determiner::An,
            "the"   => Determiner::The,
            "auto"  => Determiner::Auto,
            _       => Determiner::Blank,
        }
    }
}

impl Default for Determiner {
    fn default() -> Determiner  { Determiner::Blank }
}
