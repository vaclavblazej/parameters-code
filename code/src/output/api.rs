use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::data::data::{Data, Parameter};
use crate::data::enums::*;
use crate::data::id::*;
use crate::data::preview;
use crate::data::preview::PreviewParameter;
use crate::general::file;
use crate::work::combine;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleApiSet {
    pub id: String,
    pub name: String,
    pub score: u32,
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

impl From<&PreviewParameter> for SimpleApiSet {
    fn from(raw: &PreviewParameter) -> Self {
        SimpleApiSet {
            id: raw.id.to_string(),
            name: raw.name_core.name.clone(),
            score: raw.score,
        }
    }
}

impl From<&(PreviewParameter, PreviewParameter)> for SimpleApiRelation {
    fn from((subset, superset): &(PreviewParameter, PreviewParameter)) -> Self {
        SimpleApiRelation {
            subset_id: subset.id.to_string(),
            superset_id: superset.id.to_string(),
        }
    }
}

impl From<&Data> for SimpleApiData {
    fn from(raw: &Data) -> Self {
        // let init_relations = raw
        //     .relations
        //     .iter()
        //     .filter_map(|r| match r.cpx {
        //         SourcedCpxInfo::Equal { .. } | SourcedCpxInfo::Inclusion { .. } => {
        //             Some(PreviewRelation {
        //                 id: r.id.preview(),
        //                 subset: r.subset.clone(),
        //                 superset: r.superset.clone(),
        //                 cpx: CpxInfo::Inclusion {
        //                     mn: None,
        //                     mx: Some(CpxTime::Linear),
        //                 },
        //             })
        //         }
        //         SourcedCpxInfo::Exclusion { .. } | SourcedCpxInfo::Unknown => None,
        //     })
        //     .collect();
        // let preview_sets = raw.parameters.keys().cloned().collect();
        // let relations = init_relations
        //     .iter()
        //     .map(|x| SimpleApiRelation {
        //         subset_id: x.subset.id.to_string(),
        //         superset_id: x.superset.id.to_string(),
        //     })
        //     .collect();
        // let sets = raw
        //     .parameters
        //     .values()
        //     .map(|x: &Parameter| x.preview())
        //     .map(SimpleApiSet::from)
        //     .collect();
        SimpleApiData {
            date: format!("{}", chrono::Local::now().format("%Y-%m-%d")),
            sets: vec![], // todo
            relations: vec![],
        }
    }
}

pub fn create_simple_api(data: &Data, api_dir: &Path) -> Result<()> {
    let simple_data = SimpleApiData::from(data);
    let serialized = serde_json::to_string_pretty(&simple_data)?;
    let final_file = api_dir.join("simple.json");
    file::write_file_content(&final_file, serialized.as_str())?;
    Ok(())
}

pub fn create_set_api(data: &Data, api_dir: &Path) -> Result<()> {
    for parameter in data.parameters.values() {
        let serialized = serde_json::to_string_pretty(parameter)?;
        let filename = format!("{}.json", parameter.id());
        let final_file = api_dir.join(filename);
        file::write_file_content(&final_file, serialized.as_str())?;
    }
    Ok(())
}
