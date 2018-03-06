#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate html5ever;
#[cfg(feature = "reqwest")]
extern crate reqwest;

mod object;
mod image;
mod audio;
mod video;

pub mod scraper;
pub mod error;

pub use object::Object;
pub use image::Image;
pub use audio::Audio;
pub use video::Video;
pub use scraper::scrape;
pub use scraper::extract;
pub use scraper::{
    extract_open_graph_from_meta_tag,
    extract_image,
    extract_audio,
    extract_video
};
