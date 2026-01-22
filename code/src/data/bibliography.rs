use std::path::PathBuf;

use anyhow::Result;
use biblatex::{Bibliography, Chunk, DateValue, PermissiveType, Spanned};
use log::error;

use crate::{
    data::{date::Date, enums::SourceKey},
    general::{file, strings::nice_concat},
};

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
    raw_entry_key: &str,
) -> (SourceKey, Date) {
    let mut name: Option<String> = None;
    let mut entry: Option<String> = None;
    let mut date = Date::empty();
    if let Some(bib) = bibliography {
        bib.get(raw_entry_key).map(|e| {
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
                name = Some(title_str);
            }
            if let Ok(fauthors) = e.author() {
                let sauthors: Vec<String> = fauthors.iter().map(|x| x.name.clone()).collect();
                let authors = nice_concat(sauthors);
                match name.clone() {
                    Some(mut q) => {
                        q.push_str(&format!(" by {}", authors));
                        name = Some(q);
                    }
                    None => name = Some(authors),
                };
            }
            if let Ok(dt) = e.date() {
                match dt {
                    PermissiveType::Typed(t) => match t.value {
                        DateValue::At(d) => {
                            date.year = Some(d.year);
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
        });
    }
    (
        SourceKey::Bibtex {
            entry_key: String::from(raw_entry_key),
            name,
            entry_content: entry,
        },
        date,
    )
}
