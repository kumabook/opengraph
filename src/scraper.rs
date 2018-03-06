use std::io::Read;
use error::Error;

use html5ever::rcdom::NodeData::{
    Document,
    Doctype,
    Text,
    Comment,
    Element,
    ProcessingInstruction
};
use html5ever::rcdom::{RcDom, Handle};
use html5ever::{parse_document, Attribute};
use html5ever::tendril::TendrilSink;

#[cfg(feature = "reqwest")]
use std::time::Duration;
#[cfg(feature = "reqwest")]
use reqwest;

use Object;
use Image;
use Audio;
use Video;

pub struct Opts {
    include_images: bool,
    include_audios: bool,
    include_videos: bool,
}

impl Default for Opts {
    fn default() -> Opts {
        Opts {
            include_images: false,
            include_videos: false,
            include_audios: false,
        }
    }
}

#[cfg(feature = "reqwest")]
pub fn scrape(url: &str, option: Opts) -> Result<Object, Error> {
    let client = reqwest::Client::builder()
        .timeout(Duration::new(30, 0))
        .build()?;
    let mut res = client.get(url).send()?;
    if res.status().is_success() {
        extract(&mut res, option).map(|mut obj| {
            obj.images = obj.images.iter().map(|i| {
                let mut i = i.clone();
                i.normalize(&res.url());
                i
            }).collect::<Vec<Image>>();
            obj
        })
    } else {
        Err(Error::Unexpected)
    }
}

pub fn extract<R>(input: &mut R, option: Opts) -> Result<Object, Error> where R: Read {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(input)
        .unwrap();
    let mut og_props = Vec::new();
    let mut images   = Vec::new();
    let mut audios   = Vec::new();
    let mut videos   = Vec::new();
    walk(dom.document,
         &mut og_props,
         &mut images,
         &mut audios,
         &mut videos,
         &option);
    let mut obj = Object::new(&og_props);
    obj.images.append(&mut images);
    obj.audios.append(&mut audios);
    obj.videos.append(&mut videos);
    Ok(obj)
}

fn walk(handle:    Handle,
        og_props:  &mut Vec<(String, String)>,
        images:    &mut Vec<Image>,
        audios:    &mut Vec<Audio>,
        videos:    &mut Vec<Video>,
        option:    &Opts) {
    match handle.data {
        Document       => (),
        Doctype { .. } => (),
        Text { .. }    => (),
        Comment { .. } => (),
        Element { ref name, ref attrs, ..} => {
            let tag_name = name.local.as_ref();
            match tag_name {
                "meta" => {
                    let mut ps = extract_open_graph_from_meta_tag(&attrs.borrow());
                    og_props.append(&mut ps);
                },
                "img" => {
                    if option.include_images {
                        if let Some(image) = extract_image(&attrs.borrow()) {
                            images.push(image);
                        }
                    }
                },
                "audio" => {
                    if option.include_audios {
                        if let Some(audio) = extract_audio(&attrs.borrow()) {
                            audios.push(audio);
                        }
                    }
                },
                "video" => {
                    if option.include_videos {
                        if let Some(video) = extract_video(&attrs.borrow()) {
                            videos.push(video);
                        }
                    }
                },
                _ => (),
            }
        },
        ProcessingInstruction { .. } => unreachable!()
    }
    for child in handle.children.borrow().iter() {
        walk(child.clone(), og_props, images, audios, videos, option)
    }
}

fn attr(attr_name: &str, attrs: &Vec<Attribute>) -> Option<String> {
    for attr in attrs.iter() {
        if attr.name.local.as_ref() == attr_name {
            return Some(attr.value.to_string())
        }
    }
    None
}

pub fn extract_open_graph_from_meta_tag(attrs: &Vec<Attribute>) -> Vec<(String, String)> {
    let mut og_props = vec!();
    match extract_open_graph_prop("property", attrs) {
        Some((key, content)) => og_props.push((key, content)),
        None                 => (),
    }
    match extract_open_graph_prop("name", attrs) {
        Some((key, content)) => og_props.push((key, content)),
        None                 => (),
    }
    og_props
}

fn extract_open_graph_prop<'a>(attr_name: &str, attrs: &Vec<Attribute>) -> Option<(String, String)> {
    attr(attr_name, attrs)
        .and_then(|property|
                  if property.starts_with("og:") {
                      let end = property.chars().count();
                      let key = unsafe {
                          property.slice_unchecked(3, end)
                      }.to_string();
                      attr("content", attrs).map(|content| (key, content))
                  } else {
                      None
                  })
}

pub fn extract_image(attrs: &Vec<Attribute>) -> Option<Image> {
    attr("src", attrs).map(|src| Image::new(src.to_string()))
}

pub fn extract_audio(attrs: &Vec<Attribute>) -> Option<Audio> {
    attr("src", attrs).map(|src| Audio::new(src.to_string()))
}

pub fn extract_video(attrs: &Vec<Attribute>) -> Option<Video> {
    attr("src", attrs).map(|src| Video::new(src.to_string()))
}

#[cfg(test)]
mod test {
    use super::*;
    use object::ObjectType;
    #[test]
    fn extract_open_graph_object() {
        let x = r#"
<html prefix="og: http://ogp.me/ns#">
<head>
<title>The Rock (1996)</title>
<meta property="og:title" content="The Rock" />
<meta property="og:type" content="video.movie" />
<meta property="og:url" content="http://www.imdb.com/title/tt0117500/" />
<meta property="og:image" content="http://ia.media-imdb.com/images/rock.jpg" />
</head>
</html>
                "#;
        let obj = extract(&mut x.to_string().as_bytes(), Default::default());
        assert!(obj.is_ok());
        let obj = obj.unwrap();
        assert_eq!(&obj.title, "The Rock");
        assert_eq!(obj.obj_type, ObjectType::Movie);
        assert_eq!(&obj.url, "http://www.imdb.com/title/tt0117500/");
        assert_eq!(obj.images.len(), 1);
        assert_eq!(&obj.images[0].url, "http://ia.media-imdb.com/images/rock.jpg");
    }
}
