use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use matter::matter;
use pulldown_cmark::{html, Options, Parser};
use serde::Serialize;
use serde_yaml::{self, Mapping};
use std::cmp::Ordering;
use std::fmt::Error;
use std::fs::DirEntry;
use std::path::PathBuf;

use crate::highlight;

#[derive(Debug)]
pub struct NoteError {
    details: String,
}

impl NoteError {
    pub fn new(msg: &str) -> NoteError {
        NoteError { details: msg.to_string() }
    }
}

impl From<std::io::Error> for NoteError {
    fn from(e: std::io::Error) -> NoteError {
        let message = format!("{}", e);
        NoteError::new(&message)
    }
}

#[derive(Debug, Clone, Eq, Serialize)]
pub struct Note {
    pub path: PathBuf,
    pub date: DateTime<Utc>,
    pub title: String,
    pub content: Option<String>,
    pub frontmatter: Mapping,
    pub slug: String,
}

impl Note {
    pub fn from(path: DirEntry) -> Result<Note, NoteError> {

        println!("Converting: {:?}", path.path());

        let input = std::fs::read_to_string(&path.path()).unwrap();
        if let Some((matter, _markdown)) = matter(&input) {
            let frontmatter = make_frontmatter(matter).unwrap();
            Ok(Note {
                path: path.path(),
                title: String::from(frontmatter["title"].as_str().unwrap()),
                date: date_from_headers(&frontmatter).unwrap(),
                frontmatter,
                content: None,
                slug: path.path().file_stem().unwrap().to_string_lossy().to_string() + ".html"
            })
        } else {
            Err(NoteError {
                details: "Did not work".to_string(),
            })
        }
    }

    // pub fn parse_content(& mut self) {
    //     if let Some((matter, markdown)) = matter(&self.path.to_str().unwrap()) {
    //       self.content = Some(make_markdown_content(markdown).unwrap().to_string());
    //     }
    // }

    pub fn parse_content(self: &Note) -> Result<Option<String>, NoteError>{

        // TODO reading file twice :( or we store this when calling matter above?
        let input = std::fs::read_to_string(&self.path).unwrap();

        if let Some((_matter, markdown)) = matter(&input) {
          Ok(Some(make_markdown_content(markdown).unwrap().to_string()))
        }else{
            //NoteError::new("Could not parse")
            panic!("Could not parse")
        }
    }

    pub fn summary(self: &Note) -> String {
        self.title.to_string()
    }
}


impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.title == other.title
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

fn date_from_headers(frontmatter: &Mapping) -> Result<DateTime<Utc>, NoteError> {
    let date_string = frontmatter
        .get("date")
        .ok_or_else(|| NoteError::new("Missing date"))?;

    let date = date_string.as_str().unwrap().replace("\"", "");
    let naive_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|_| NoteError::new("Invalid date"))?;
    let midnight = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();
    Ok(DateTime::<Utc>::from_utc(
        naive_date.and_time(midnight),
        Utc,
    ))
}

fn make_frontmatter(matter: String) -> Result<Mapping, Error> {
    let m: Mapping = serde_yaml::from_str(&matter).unwrap();
    Ok(m)
}

fn with_delim_removed(html: String) -> String {
    str::replace(&html, "DELIM", "")
}


fn make_markdown_content(markdown: String) -> Result<String, Error> {
    // TODO factor this out into global scope somehow.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(&markdown, options);

    let mut html_output = String::with_capacity(4000);
    html::push_html(&mut html_output, parser);


    Ok(with_delim_removed(highlight::with_highlighted_code_snippets(&html_output)).to_string())
}
