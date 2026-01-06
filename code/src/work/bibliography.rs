use std::path::PathBuf;

use anyhow::Result;
use biblatex::Bibliography;
use log::error;

use crate::general::file;


pub fn load_bibliography(bibliography_file: &PathBuf) -> Result<Bibliography> {
    let bibliography_res = file::read_file_content(bibliography_file);
    match bibliography_res {
        Ok(bibliography_str) => Ok(Bibliography::parse(&bibliography_str).unwrap()),
        Err(error) => {
            error!("cannot load bibliography from {:?}", bibliography_file);
            Err(error)
        }
    }
}

pub fn bibligraphy_to_source(
    bibliography: &Option<Bibliography>,
    time: &mut Date,
    key: &String,
) -> SourceKey::Bibtex {
    let mut name: Option<String> = None;
    let entry = bibliography.map(|bib| {
        bib.get(key).map(|e| {
            if let Ok(title) = e.title() {
                let title_str: String = title
                    .iter()
                    .map(|Spanned { v: chunk, span: _ }| match chunk {
                        Chunk::Normal(value) => value.clone(),
                        Chunk::Verbatim(value) => format!("`{}`", value),
                        Chunk::Math(value) => format!("${}$", value),
                    })
                    .fold("".into(), |mut a, b| {
                        a.push_str(&b);
                        a
                    });
                name = Some(match name {
                    Some(mut q) => {
                        q.push_str(&title_str);
                        q
                    }
                    None => title_str,
                });
            }
            if let Ok(fauthors) = e.author() {
                let sauthors: Vec<String> =
                    fauthors.iter().map(|x| x.name.clone()).collect();
                let authors = sauthors.join(", ");
                name = Some(match name {
                    Some(mut q) => {
                        q.push_str(&format!(" by {}", authors));
                        q
                    }
                    None => authors,
                });
            }
            if let Ok(dt) = e.date() {
                match dt {
                    PermissiveType::Typed(t) => match t.value {
                        DateValue::At(d) => {
                            time = Date {
                                year: Some(d.year),
                                month: None,
                                day: None,
                            }
                        }
                        DateValue::After(d) => {
                            panic!("unknown date type")
                        }
                        DateValue::Before(d) => {
                            panic!("unknown date type")
                        }
                        DateValue::Between(s, e) => {
                            panic!("unknown date type")
                        }
                    },
                    PermissiveType::Chunks(chunks) => {
                        panic!("unknown date type")
                    }
                }
            }
            Some(e.to_biblatex_string())
        })
    });
    SourceKey::Bibtex {
        key: key.clone(),
        name,
        entry,
        relevance: source.relevance,
    }
}
