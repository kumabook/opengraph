use html5ever::rcdom::{Document, Doctype, Text, Comment, Element};
use html5ever::rcdom::{RcDom, Handle};
use html5ever::{parse_document, Attribute};
use html5ever::tendril::TendrilSink;

use Object;

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
        og_props:  &mut Vec<(String, String)>) {
    let node = handle.borrow();
    match node.node {
        Document         => (),
        Doctype(_, _, _) => (),
        Text(_)          => (),
        Comment(_)       => (),
        Element(ref name, _, ref attrs) => {
            let tag_name = name.local.as_ref();
            let mut ps = extract_open_graph_metadata_from_tag(tag_name, attrs);
            og_props.append(&mut ps);
        }
    }
    for child in node.children.iter() {
        walk(child.clone(), og_props);
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

fn extract_open_graph_metadata_from_tag(tag_name: &str,
                                            attrs: &Vec<Attribute>) -> Vec<(String, String)> {
    let mut og_props = vec!();
    if tag_name == "meta" {
        match extract_open_graph_prop("property", attrs) {
            Some((key, content)) => og_props.push((key, content)),
            None                 => (),
        }
        match extract_open_graph_prop("name", attrs) {
            Some((key, content)) => og_props.push((key, content)),
            None                 => (),
        }
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
