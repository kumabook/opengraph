opengraph
=========

<img height="260" src="http://ogp.me/logo.png">

opengraph is a library that parses html and extracts Open Graph protocol markup.


Usage
-----


- Add `opengraph` to dependencies in Cargo.toml

```toml
[dependencies]
opengraph = "^0"
```

- Then, use `opengraph::scrape` as below:

```rust

extern crate opengraph;

fn main() {
  match opengraph::scrape("https://spincoaster.com/chromeo-juice", Default::default()) {
      Ok(object) => {
          println!("{:?}", object);
      },
      Err(_) => println!("error occured"),
  }
}

```

or use `opengraph::extract` as below:

```rust
extern crate opengraph;

fn main() {
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
    match opengraph::extract(&mut x.to_string().as_bytes(), Default::default()) {
        Ok(object) => {
            println!("{:?}", object);
        },
        Err(_) => println!("error occured"),
    }
}
```

License
-------

[MIT](LICENSE)
