
#[macro_use] extern crate lazy_static;
use std::collections::HashMap;
use std::ffi::OsStr;
use note::NoteError;
use tera::Tera;
use std::fs;
use std::path::PathBuf;
mod highlight;
pub mod note;
use std::env;
use note::Note;
use std::cmp::Reverse;
use tera::Context;
use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;
use chrono::SecondsFormat;

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


//static BASE_URL: &str =  "/Users/nico/Dev/blog-engine2/public";
static BASE_URL: &str = "https://notes.embedded-pepper.dev";


fn make_index(tera: &Tera, notes: &Vec<Note>){


    let count = notes.len();

    let mut index = 20;
    if index > count {
        index = count;
    }

    let mut context = Context::new();
    let top20 = &notes[..index];
    context.insert("notes", &top20);
    context.insert("title", "Nico's Note Index");
    context.insert("base_url", &BASE_URL);


    let out = tera.render("index.html", &context).unwrap();

    let dir = env::current_dir().unwrap();
    let p = dir.join("./public").join("index").with_extension("html");
    std::fs::write(p, out).map_err(|_| NoteError::new("Could not make Index"));

    println!("Done making Index Page");

}

fn make_tags(tera: &Tera, taxonomy: &HashMap<String, Vec<&Note>>){


    for (key, value) in taxonomy{

        let mut context = Context::new();
        context.insert("notes", &value);
        context.insert("title", key);
        context.insert("base_url", &BASE_URL);

        let out = tera.render("tags.html", &context).unwrap();
        let dir = env::current_dir().unwrap();
        let p = dir.join("./public").join("tags").join(key).with_extension("html");
        std::fs::write(p, out).map_err(|_| NoteError::new("Could not make taxonomy"));

        println!("Done making Tags Page: {:?}", key.to_string())
    }


}

fn make_about(tera: &Tera){
    let mut context = Context::new();
    context.insert("base_url", &BASE_URL);

    let out = tera.render("about.html", &context).unwrap();
    let dir = env::current_dir().unwrap();
    let p = dir.join("./public").join("about").with_extension("html");
    std::fs::write(p, out).map_err(|_| NoteError::new("Could not make taxonomy"));
    println!("Done making About Page")

}

fn make_rss_feed(tera: &Tera, notes: &Vec<Note>) {
    let mut context = Context::new();
    context.insert("notes", &notes);
    context.insert("title", "Nico's notes");
    context.insert("description", "Nico's notes");

    context.insert("feed_url", "notes.embedded-pepper.dev/rss.xml");

    context.insert("lang", "en");

    context.insert("base_url", &BASE_URL);

    let system_time = SystemTime::now();
    let now: DateTime<Utc> = system_time.into();
    let now = now.to_rfc3339_opts(SecondsFormat::Secs, true);
    context.insert("last_updated", &now);

    let out = tera.render("rss.html", &context).unwrap();
    let dir = env::current_dir().unwrap();
    let p = dir.join("./public").join("rss").with_extension("xml");
    std::fs::write(p, out).map_err(|_| NoteError::new("Could not make rss"));
    println!("Done making Rss Xml")
}

fn make_sitemap(tera: &Tera, notes: &Vec<Note>) {
    let mut context = Context::new();
    context.insert("base_url", &BASE_URL);
    context.insert("notes", &notes);

    let out = tera.render("sitemap.html", &context).unwrap();
    let dir = env::current_dir().unwrap();
    let p = dir.join("./public").join("sitemap").with_extension("xml");
    std::fs::write(p, out).map_err(|_| NoteError::new("Could not make Sitemap"));
    println!("Done making Sitemap Page")

}


fn main() {
    println!("The engine - generate the notes html from markdown !");

    // // Use globbing
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };


    let dir = env::current_dir().unwrap().join("notes");
    let notes = all_notes(dir).unwrap();
    let mut taxonomy: HashMap<String, Vec<&Note>> = HashMap::new();

    for note in &notes{
        // Using the tera Context struct

        let html_highlighted = note.parse_content().unwrap();
        let mut context = Context::new();
        context.insert("content", &html_highlighted);
        context.insert("frontmatter", &note.frontmatter);
        context.insert("base_url", &BASE_URL);

        let out = tera.render("notes.html", &context).unwrap();

        let dir = env::current_dir().unwrap();
        let p = dir.join("./public").join("notes").join( &note.slug);
        std::fs::write(p, out);

        if note.frontmatter.contains_key("tags"){
            let tags = &note.frontmatter.get("tags").unwrap();
            for t in tags.as_sequence().unwrap(){
                let key = t.as_str().unwrap().to_lowercase().as_str().to_owned();
                let value = &note;
                if taxonomy.contains_key(&key){
                    let v = taxonomy.get_mut(&key).unwrap();
                    v.push(value);
                }else{
                    taxonomy.insert(key, vec![value]);
                }
            }
        }
    }
    make_index(&tera, &notes);
    make_tags(&tera, &taxonomy);
    make_about(&tera);
    make_rss_feed(&tera, &notes);
    make_sitemap(&tera, &notes);


    // styling
    // how to serve

    // publix miniflux reading list

}
