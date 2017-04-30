use html5ever::rcdom::{Document, Doctype, Text, Comment, Element};
use html5ever::rcdom::{RcDom, Handle};
use html5ever::{parse_document, Attribute};
use html5ever::tendril::TendrilSink;

use Object;
use Image;
use Audio;
use Video;

pub fn extract(html: String) -> Option<Object> {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();
    let mut og_props  = Vec::new();
    walk(dom.document, &mut og_props);
    if og_props.len() > 0 {
        Some(Object::new(&og_props))
    } else {
        None
    }
}

fn walk(handle:    Handle,
        og_props:  &mut Vec<(String, String)>,
        images:    &mut Vec<Image>,
        audios:    &mut Vec<Audio>,
        videos:    &mut Vec<Video>) {
    let node = handle.borrow();
    match node.node {
        Document         => (),
        Doctype(_, _, _) => (),
        Text(_)          => (),
        Comment(_)       => (),
        Element(ref name, _, ref attrs) => {
            let tag_name = name.local.as_ref();
            match tag_name {
                "meta" => {
                    let mut ps = extract_open_graph_from_meta_tag(attrs);
                    og_props.append(&mut ps);
                },
                "img" => {
                    if let Some(image) = extract_image(attrs) {
                        images.push(image);
                    }
                },
                "audio" => {
                    if let Some(audio) = extract_audio(attrs) {
                        audios.push(audio);
                    }
                },
                "videos" => {
                    if let Some(video) = extract_video(attrs) {
                        videos.push(video);
                    }
                },
                _ => (),
            }
        }
    }
    for child in node.children.iter() {
        walk(child.clone(), og_props, images, audios, videos)
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

fn extract_open_graph_from_meta_tag(attrs: &Vec<Attribute>) -> Vec<(String, String)> {
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

fn extract_image(attrs: &Vec<Attribute>) -> Option<Image> {
    attr("src", attrs).map(|src| Image::new(src.to_string()))
}

fn extract_audio(attrs: &Vec<Attribute>) -> Option<Audio> {
    attr("src", attrs).map(|src| Audio::new(src.to_string()))
}

fn extract_video(attrs: &Vec<Attribute>) -> Option<Video> {
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
        let obj = extract(x.to_string());
        assert!(obj.is_some());
        let obj = obj.unwrap();
        assert_eq!(&obj.title, "The Rock");
        assert_eq!(obj.obj_type, ObjectType::Movie);
        assert_eq!(&obj.url, "http://www.imdb.com/title/tt0117500/");
        assert_eq!(obj.images.len(), 1);
        assert_eq!(&obj.images[0].url, "http://ia.media-imdb.com/images/rock.jpg");
    }
}
