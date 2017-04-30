#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate html5ever;
extern crate hyper;
extern crate hyper_native_tls;

mod object;
mod image;
mod audio;
mod video;

pub mod scraper;

pub use object::Object;
pub use image::Image;
pub use audio::Audio;
pub use video::Video;
pub use scraper::scrape;
pub use scraper::extract;
