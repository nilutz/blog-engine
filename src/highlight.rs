extern crate syntect;

use std::env;
use std::path::PathBuf;

use regex::Regex;
use regex::RegexBuilder;
use regex::Captures;

use self::syntect::parsing::{SyntaxReference, SyntaxSet};
use self::syntect::highlighting::{Theme, ThemeSet};
use self::syntect::html::highlighted_html_for_string;
use self::syntect::dumps::from_binary;

pub fn highlighted_html_for(snippet: &String, lang: Option<String>) -> String {
    match lang {
        Some(lang) => highlighted_html_for_language(&snippet, lang),
        None => snippet.to_string()
    }
}

pub fn highlighted_html_for_language(snippet: &String, attributes: String) -> String {
    lazy_static! {
        static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
        // from_binary(include_bytes!("../sublime/syntaxes/newlines.packdump"));

        //SyntaxSet::load_defaults_newlines();

        static ref THEME: Theme = ThemeSet::get_theme(theme_path().as_path()).unwrap();
        static ref PYTHON_SYNTAX: &'static SyntaxReference = SYNTAX_SET.find_syntax_by_extension("py").unwrap();
        static ref RUST_SYNTAX: &'static SyntaxReference = SYNTAX_SET.find_syntax_by_extension("rs").unwrap();
        static ref SQL_SYNTAX: &'static SyntaxReference = SYNTAX_SET.find_syntax_by_extension("sql").unwrap();
    }

    if attributes.contains("python") {
        highlighted_html_for_string(&snippet, &SYNTAX_SET, &PYTHON_SYNTAX, &THEME)
    } else if attributes.contains("rust") {
        highlighted_html_for_string(&snippet, &SYNTAX_SET, &RUST_SYNTAX, &THEME)
    } else if attributes.contains("sql") {
        highlighted_html_for_string(&snippet, &SYNTAX_SET, &SQL_SYNTAX, &THEME)
    } else {
        format!("<pre><code {}>{}</code></pre>", attributes, snippet.to_string())
    }
}

fn theme_path() -> PathBuf {
    // let dir = env::current_dir().unwrap();
    // dir.join("theme").join("pat.tmTheme")
    env::current_dir().unwrap().join("./sublime/themes/agola-dark.tmTheme")

}

fn syntax_path() -> PathBuf {
    env::current_dir().unwrap().join("../sublime/syntaxes")
}


pub fn with_highlighted_code_snippets(html: &String) -> String {
    lazy_static! {
        static ref CODE_SNIPPET: Regex = RegexBuilder::new("<pre><code([^>]*)>(.*?)</code></pre>")
                                            .dot_matches_new_line(true)
                                            .unicode(true)
                                            .build()
                                            .unwrap();

    }

    CODE_SNIPPET.replace_all(html, |captures: &Captures| {

        let attributes = captures.get(1).map(|m| m.as_str().to_string());
        let snippet = captures.get(2).map_or("", |m| m.as_str()).to_string();
        let mut trimmed_snippet = snippet.trim().to_string();
        let attribute = attributes.to_owned();

        // Manually escaping :(
        trimmed_snippet = trimmed_snippet.replace("&lt;", "<");
        trimmed_snippet = trimmed_snippet.replace("&gt;", ">");
        trimmed_snippet = trimmed_snippet.replace("&amp;", "&");
        trimmed_snippet = trimmed_snippet.replace("&quot;", r#"""#);


        highlighted_html_for(&trimmed_snippet, attributes)

    }).to_string()
}