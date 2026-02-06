use serde::{Deserialize, Serialize};

use crate::data::{
    data::{Named, ProviderLink},
    enums::SourceKey,
    id::HasId,
    preview::{
        PreviewGraphClass, PreviewParameter, PreviewParametricGraphClass, PreviewSource, PreviewTag,
    },
};

fn html_base(id: &String) -> String {
    format!("{{{{< base >}}}}html/{}", id)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash, Eq)]
pub struct Link {
    pub url: String,
    pub name: String,
}

pub trait Linkable {
    fn get_link(&self) -> Link;
}

// impl Linkable for PreviewRelation {
//     fn get_url(&self) -> String {
//         html_base(&format!("relations#{}", &self.id.to_string()))
//     }
//     fn get_name(&self) -> String {
//         format!("{} → {}", self.subset.name, self.superset.name)
//     }
// }

impl<T> Linkable for T
where
    T: HasId + Named,
{
    fn get_link(&self) -> Link {
        Link {
            url: html_base(&self.id()),
            name: self.name_core().name.clone(),
        }
    }
}

impl Linkable for PreviewTag {
    fn get_link(&self) -> Link {
        Link {
            url: html_base(&self.id.to_string()),
            name: self.name.name.clone(),
        }
    }
}

impl Linkable for PreviewGraphClass {
    fn get_link(&self) -> Link {
        Link {
            url: html_base(&self.id.to_string()),
            name: self.name_core.name.clone(),
        }
    }
}

impl Linkable for PreviewParametricGraphClass {
    fn get_link(&self) -> Link {
        Link {
            url: html_base(&self.id.to_string()),
            name: self.name_core.name.clone(),
        }
    }
}

impl Linkable for PreviewParameter {
    fn get_link(&self) -> Link {
        Link {
            url: html_base(&self.id.to_string()),
            name: self.name_core.name.clone(),
        }
    }
}

impl Linkable for PreviewSource {
    fn get_link(&self) -> Link {
        let url = match &self.sourcekey {
            SourceKey::Bibtex {
                entry_key: _,
                name: _,
                entry_content: _,
            } => html_base(&self.id.to_string()),
            SourceKey::Online { url } => url.clone(),
            SourceKey::Other {
                name: _,
                description: _,
            } => html_base(&self.id.to_string()),
        };
        let name = match &self.sourcekey {
            SourceKey::Bibtex {
                entry_key: key,
                name,
                entry_content: _,
            } => name.clone().unwrap_or(key.clone()),
            SourceKey::Online { url } => url.clone(),
            SourceKey::Other {
                name,
                description: _,
            } => name.into(),
        };
        Link { url, name }
    }
}
