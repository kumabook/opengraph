#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod object;
mod image;
mod audio;
mod video;

pub use object::Object;
pub use image::Image;
pub use audio::Audio;
pub use video::Video;
