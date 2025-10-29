
use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::data::preview::{PreviewSet, PreviewRelation, WorkRelation, PreviewType};
use crate::data::id::{BaseId, PreviewRelationId};
use crate::data::core::Data;
use crate::work::combine;
use crate::general::file;
use crate::general::hide::filter_hidden;
use crate::general::enums::{SourcedCpxInfo, CpxInfo, CpxTime};


#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleApiSet {
    pub id: String,
    pub name: String,
    pub relevance: u32,
    pub typ: PreviewType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleApiRelation {
    pub subset_id: String,
    pub superset_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleApiData {
    pub date: String,
    pub sets: Vec<SimpleApiSet>,
    pub relations: Vec<SimpleApiRelation>,
}


impl From<&PreviewSet> for SimpleApiSet {
    fn from(raw: &PreviewSet) -> Self {
        SimpleApiSet {
            id: raw.id.to_string(),
            name: raw.name.clone(),
            typ: raw.typ.clone(),
            relevance: raw.relevance,
        }
    }
}

impl From<&(PreviewSet, PreviewSet)> for SimpleApiRelation {
    fn from((subset, superset): &(PreviewSet, PreviewSet)) -> Self {
        SimpleApiRelation {
            subset_id: subset.id.to_string(),
            superset_id: superset.id.to_string(),
        }
    }
}

impl From<&Data> for SimpleApiData {
    fn from(raw: &Data) -> Self {
        let initial_relations = raw.relations.iter().filter_map(|r|{
                match r.cpx {
                    SourcedCpxInfo::Equal { .. } | SourcedCpxInfo::Inclusion { .. } => {
                        Some(PreviewRelation {
                            id: r.id.preview(),
                            subset: r.subset.clone(),
                            superset: r.superset.clone(),
                            cpx: CpxInfo::Inclusion {
                                mn: None,
                                mx: Some(CpxTime::Linear),
                            },
                        })
                    },
                    SourcedCpxInfo::Exclusion { .. } | SourcedCpxInfo::Unknown => {
                        None
                    }
                }
            }).collect();
        let preview_sets = raw.set_idx.iter().map(|(k,_)|k.clone()).collect();
        let shown_relations = filter_hidden(initial_relations, &preview_sets);
        let relations = shown_relations.iter().map(|x|{
            SimpleApiRelation{
                subset_id: x.subset.id.to_string(),
                superset_id: x.superset.id.to_string(),
            }
        }).collect();
        let sets = raw.set_idx.iter().map(|(k,_)|SimpleApiSet::from(k)).collect();
        SimpleApiData {
            date: format!("{}", chrono::Local::now().format("%Y-%m-%d")),
            sets,
            relations,
        }
    }
}


pub fn create_simple_api(data: &Data, api_dir: &PathBuf) -> Result<()> {
    let simple_data = SimpleApiData::from(data);
    let serialized = serde_json::to_string_pretty(&simple_data)?;
    let final_file = api_dir.join("simple_api.json");
    file::write_file_content(&final_file, serialized.as_str())?;
    Ok(())
}

pub fn create_set_api(data: &Data, api_dir: &PathBuf) -> Result<()> {
    for set in &data.sets {
        let serialized = serde_json::to_string_pretty(set)?;
        let filename = format!("{}.json", set.id);
        let final_file = api_dir.join(filename);
        file::write_file_content(&final_file, serialized.as_str())?;
    }
    Ok(())
}
