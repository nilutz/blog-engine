---
title: Hello word! Hello Rust!
description: A first entry to this blog! And the underlying mechanisms.
author: Nico Lutz
date: 2023-02-06
tags:
    - Rust
---

## A Blog engine

This is a simple static site generator. The work is based on [Zola](https://www.getzola.org/) and [this](https://patshaughnessy.net/2019/9/4/using-rust-to-build-a-blog-site) blog post by Pat Shaughnessy. I wanted to do a blog for a long time and I also wanted to learn rust for a long time. So I started learning rust by building this static site generator, which in turn powers this blog. The engine takes markdown files in `/notes` and  [tera](https://tera.netlify.app) templates (located in `/templates`) then it turns them into plain old html. Simple as that.

The rest of this note shows basic functionality on how the html pages are generated. The heart of the engine is a function that iterates the `/notes` directory and generates a struct containing necessary metadata:

```rust
fn all_notes(input_path: PathBuf) -> Result<Vec<Note>, NoteError>{

    let paths = fs::read_dir(&input_path).unwrap();
    let all_posts: Result<Vec<Note>, NoteError> = paths.filter_map(Result::ok)
         .filter(|f| f.path().extension().and_then(OsStr::to_str) == Some("md"))
         .map(|p| Note::from(p) )
         .collect();
    let mut all_posts = all_posts?;
    all_posts.sort_by_key(|p| Reverse(p.date));

    Ok(all_posts)
}
```

The `Note` struct:

```rust
pub struct Note {
    pub path: PathBuf,
    pub date: DateTime<Utc>,
    pub title: String,
    pub content: Option<String>,
    pub frontmatter: Mapping,
    pub slug: String,
}
```


Anyway, then we simply iterate through the `Vec` and generate the htmls with the help of tera templates:

```rust
let html_highlighted = note.parse_content().unwrap();
let mut context = Context::new();
context.insert("content", &html_highlighted);
context.insert("frontmatter", &note.frontmatter);
context.insert("base_url", &BASE_URL);

let out = tera.render("notes.html", &context).unwrap();

let dir = env::current_dir().unwrap();
let p = dir.join("./public").join("notes").join( &note.slug);
std::fs::write(p, out);

```

But the heart is the markdown to html templates. This blog mainly deals with technical problems, naturally we want math and syntax highlighting. The `syntect` crate offers a convenient way to render syntax highlighting based on e.g. sublime plugins:

```rust
pub fn highlighted_html_for(snippet: &String, lang: Option<String>) -> String {
    match lang {
        Some(lang) => highlighted_html_for_language(&snippet, lang),
        None => snippet.to_string()
    }
}
````



