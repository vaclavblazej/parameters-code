//! Interface for a simplified input of connection information.

use std::collections::{HashMap, HashSet};

use log::error;

use crate::general::enums::{Page, TransferGroup, CpxTime::{Linear, Constant}, Cpx::UpperBound};
use super::{raw::{BuiltRawSet, BuiltRawSource, Composition, RawData, RawProvider, RawRelation, RawSet, RawShowed, RawShowedFact, RawSource, RawSourceKey, RawTag, RawType}, set::SetBuilder};
use super::source::{RawDataSource, RawDataProvider};

pub struct Builder {
    data: RawData,
    id_sanity_map: HashSet<String>,
    name_sanity_map: HashSet<String>,
    unknown_source: BuiltRawSource,
    assumed_source: BuiltRawSource,
}

impl Builder {

    pub fn new() -> Builder {
        let unknown_source = BuiltRawSource {
            id: "myit4D".into(),
            rawsourcekey: RawSourceKey::Other {
                name: "unknown source".into(),
                description: "This knowledge was added to the database without tying it to an appropriate resource.".into(),
            },
            relevance: 3,
            drawings: vec![],
        };
        let assumed_source = BuiltRawSource {
            id: "9kg0oo".into(),
            rawsourcekey: RawSourceKey::Other {
                name: "assumed".into(),
                description: "Is axiomatic knowledge from the viewpoint of HOPS website.".into(),
            },
            relevance: 6,
            drawings: vec![],
        };
        let mut data = RawData::new();
        Self {
            data,
            id_sanity_map: HashSet::new(),
            name_sanity_map: HashSet::new(),
            unknown_source,
            assumed_source,
        }
    }

    /// Retrieves all the information that was added to the builder
    /// while making the builder unusable further.
    pub fn build(mut self) -> RawData {
        RawDataSource::new(&self.unknown_source, &mut self.data).done();
        RawDataSource::new(&self.assumed_source, &mut self.data).done();
        for set in &self.data.sets.clone() {
            self.assumed_source()
                .showed("", Page::NotApplicable, &set, &set, crate::general::enums::Cpx::Equal, "assumed")
                .done();
        }
        self.data
    }

    /// Adds set to the collection while making a few sanity checks.
    pub fn add_set(&mut self, set: &RawSet) {
        if self.id_sanity_map.contains(&set.id) {
            panic!("id {} used multiple times", set.id);
        }
        if set.id.len() != 6 { // todo - polish id sanity check
            error!("id {} has non-standard format", set.id);
        }
        if self.name_sanity_map.contains(&set.name) {
            error!("name {} used multiple times", set.name);
        }
        self.data.sets.push(set.clone());
        self.id_sanity_map.insert(set.id.clone());
        self.name_sanity_map.insert(set.name.clone());
    }

    /// Represents that bounds on *from* instances transfer to *to* instances.
    /// For the bound to transfer both its endpoints must be from the same TranferGroup.
    pub fn transfers_bound_to(&mut self, group: TransferGroup, from: &RawSet, to: &RawSet) {
        let r = self.data.transfer.entry(group).or_insert_with(Vec::new);
        r.push((from.clone(), to.clone()));
    }

    /// Add a new parameter.
    /// This typically represents a defined named parameter.
    /// For ambiguous names we have no clear solution yet. (todo)
    /// For parameters with more names each can be defined as
    /// a separate parameter and then united with Equivalence.
    /// Equivalent parameters whose equivalence is to some degree surprising
    /// their definitions may be kept separate.
    pub fn parameter(&mut self, id: &str, name: &str, relevance: u32) -> SetBuilder {
        let res = BuiltRawSet::new(
            id.into(),
            name.into(),
            RawType::Parameter,
            Composition::None,
            relevance,
        );
        SetBuilder::new(res, self)
    }

    /// Add a parameter defined as the number of vertices to be removed
    /// until the remaining graph falls in the given set.
    pub fn distance_to(&mut self, id: &str, set: &RawSet, relevance: u32) -> SetBuilder {
        let res = BuiltRawSet::new(
            id.into(),
            format!("distance to {}", set.name.clone()),
            RawType::Parameter,
            Composition::None,
            relevance,
        );
        let set_local = set.clone();
        SetBuilder::new(res, self)
            .add_callback(Box::new(move|builder: &mut Builder, newset: &RawSet|{
                let mut tmp_source = builder.assumed_source();
                match &set_local.typ {
                    RawType::Parameter => tmp_source.showed("", Page::NotApplicable, &set_local, newset, UpperBound(Linear), "by definition"),
                    RawType::GraphClass => tmp_source.showed("", Page::NotApplicable, &set_local, newset, UpperBound(Constant), "by definition"),
                };
                builder.transfers_bound_to(TransferGroup::DistanceTo, &set_local, newset);
            }))
    }

    /// Create a new set that represents intersection of sets.
    /// From a view point of classical parameterized complexity
    /// we may understand the intersection as a sum of parameters.
    pub fn intersection(&mut self, id: &str, set_a: &RawSet, set_b: &RawSet, name: &str, relevance: u32) -> SetBuilder {
        let sets = vec![set_a.clone(), set_b.clone()];
        let (typ, upper_bound) = if sets.iter().all(|x|x.typ == RawType::GraphClass) {
            (RawType::GraphClass, UpperBound(Constant))
        } else {
            (RawType::Parameter, UpperBound(Linear))
        };
        let res = BuiltRawSet::new(
            id.into(),
            name.into(),
            typ,
            Composition::Intersection(sets.clone()),
            relevance,
        );
        SetBuilder::new(res, self)
            .add_callback(Box::new(move|builder: &mut Builder, newset: &RawSet|{
                let mut tmp_source = builder.assumed_source().collective(Page::NotApplicable, "by definition");
                for s in &sets {
                    let id = &format!("{}_{}", newset.id, s.id);
                    tmp_source = tmp_source.showed("", &newset, &s, upper_bound.clone());
                }
                tmp_source.done();
            }))
    }

    /// Defines a new graph class. We do not aim to have all graph
    /// classes in the database but only those that are very relevant
    /// to the field of parameterized complexity.
    pub fn graph_class(&mut self, id: &str, name: &str, relevance: u32) -> SetBuilder {
        let res = BuiltRawSet::new(
            id.into(),
            name.into(),
            RawType::GraphClass,
            Composition::None,
            relevance,
        );
        SetBuilder::new(res, self)
    }

    pub fn assumed_source(&mut self) -> RawDataSource {
        RawDataSource::new(&self.assumed_source, &mut self.data)
    }

    pub fn unknown_source(&mut self) -> RawDataSource {
        RawDataSource::new(&self.unknown_source, &mut self.data)
    }

    pub fn source(&mut self, id: &str, sourcekey: &str, relevance: u32) -> RawDataSource {
        assert!(relevance <= 9);
        let rawsourcekey = RawSourceKey::Bibtex{ key: sourcekey.into() };
        let res = BuiltRawSource { id: id.into(), rawsourcekey, relevance, drawings: vec![], };
        RawDataSource::new(&res, &mut self.data)
    }

    pub fn web_source(&mut self, id: &str, url: &str) -> RawDataSource {
        let rawsourcekey = RawSourceKey::Online{ url: url.into() };
        let res = BuiltRawSource { id: id.into(), rawsourcekey, relevance: 0, drawings: vec![], };
        RawDataSource::new(&res, &mut self.data)
    }

    /// Tie to other websites that keep information about parameters.
    pub fn provider(&mut self, name: &str, url: &str, format_url: Box<dyn Fn(&str) -> String>) -> RawDataProvider {
        RawDataProvider::new(&mut self.data, RawProvider{name: name.into(), url: url.into()}, format_url)
    }

    /// Define a tag that some sets share so they can be grouped.
    pub fn tag(&mut self, id: &str, name: &str, description: &str) -> RawTag {
        let res = RawTag {
            id: id.into(),
            name: name.into(),
            description: description.into(),
        };
        self.data.tags.push(res.clone());
        res
    }

}
