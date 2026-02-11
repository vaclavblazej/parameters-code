use std::collections::HashMap;

use anyhow::Result;
use rayon::prelude::*;

use crate::general::file;
use crate::general::progress::ProgressDisplay;
use crate::output::markdown::{Mappable, Markdown};
use crate::output::pages::TargetPage;

use super::paths::Paths;

pub(crate) fn build_page(
    page: &TargetPage,
    markdown: &Markdown,
    paths: &Paths,
) -> anyhow::Result<()> {
    let content = match page.substitute {
        Some(substitute) => substitute.object.get_page(markdown, paths),
        None => "[[handcrafted]]".into(),
    };
    let mut local_map = HashMap::new();
    let handcrafted_content = match page.source {
        Some(source) => {
            if source.as_os_str().to_str().unwrap().ends_with(".md") {
                file::read_file_content(source)?
            } else {
                let target_folder = &page.target.parent().unwrap();
                file::copy_file(source, page.target)?;
                return Ok(());
            }
        }
        None => "".into(),
    };
    local_map.insert("handcrafted", Mappable::String(handcrafted_content));
    let mut altered_content = content;
    for _ in 0..2 {
        altered_content = substitute(&altered_content, markdown, &local_map);
    }
    file::write_file_content(page.target, &altered_content)?;
    Ok(())
}

pub(crate) fn generate_pages(
    pages: &Vec<TargetPage>,
    markdown: &Markdown,
    paths: &Paths,
) -> anyhow::Result<()> {
    let progress = ProgressDisplay::new("generating pages", pages.len() as u32);
    let res: Result<Vec<()>> = pages
        .par_iter()
        .map(|page| -> anyhow::Result<()> {
            progress.increase(1);
            build_page(page, markdown, paths)
        })
        .collect();
    progress.done();
    res?;
    Ok(())
}

fn substitute(content: &str, markdown: &Markdown, map: &HashMap<&str, Mappable>) -> String {
    content
        .lines()
        .map(|line| markdown.substitute_custom_markdown(line, map))
        .collect::<Vec<String>>()
        .join("\n")
}
