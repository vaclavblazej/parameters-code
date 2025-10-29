//! Interface for a simplified input of connection information.

use std::collections::{HashMap, HashSet};

use log::error;

use super::source::{RawDataProvider, RawDataSource};
use super::{
    raw::{
        BuiltRawSet, BuiltRawSource, Composition, RawData, RawProvider, RawRelation, RawSet,
        RawShowed, RawShowedFact, RawSource, RawSourceKey, RawTag, RawType, RawOwn,
    },
    set::SetBuilder,
    sequence::SequenceBuilder,
};
use crate::{
    data::id::{BaseId, Id, PreviewId, PreviewSetId, PreviewTagId, ProviderId, ShowedId, SourceId},
    general::enums::{
        Cpx::UpperBound,
        CpxTime::{Constant, Linear},
        Page, TransferGroup,
    },
};

pub fn copyvec(vec: Vec<&PreviewSetId>) -> Vec<PreviewSetId> {
    vec.iter().map(|&x|x.to_owned()).collect()
}

pub struct Builder {
    pub data: RawData,
    id_sanity_map: HashSet<String>, // todo move unique checking to id.rs?
    name_sanity_map: HashSet<String>,
    unknown_source: RawDataSource,
    assumed_source: RawDataSource,
    set_map: HashMap<PreviewSetId, usize>, // idx to data.sets
}

pub static UNKNOWN_SOURCE_ID: &str = "myit4D";
pub static ASSUMED_SOURCE_ID: &str = "9kg0oo";

/// Defines a new graph class. We do not aim to have all graph
/// classes in the database but only those that are very relevant
/// to the field of parameterized complexity.
pub fn graph_class(id: &str, name: &str, relevance: u32) -> SetBuilder {
    let res = BuiltRawSet::new(
        id.into(),
        name.into(),
        RawType::GraphClass,
        Composition::None,
        relevance,
        );
    SetBuilder::new(res)
}

/// Defines a graph class where each graph in it is associated with an integer.
pub fn parametric_graph_class(id: &str, name: &str, relevance: u32) -> SetBuilder {
    let res = BuiltRawSet::new(
        id.into(),
        name.into(),
        RawType::GraphClass,
        Composition::None,
        relevance,
        );
    SetBuilder::new(res)
}

/// Add a new parameter.
/// This typically represents a defined named parameter.
/// For ambiguous names we have no clear solution yet. (todo)
/// For parameters with more names each can be defined as
/// a separate parameter and then united with Equivalence.
/// Equivalent parameters whose equivalence is to some degree surprising
/// their definitions may be kept separate.
pub fn parameter(id: &str, name: &str, relevance: u32) -> SetBuilder {
    let res = BuiltRawSet::new(
        id.into(),
        name.into(),
        RawType::Parameter,
        Composition::None,
        relevance,
        );
    SetBuilder::new(res)
}

pub fn parameter_sequence(id: &str) -> SequenceBuilder {
    SequenceBuilder::new(id)
}

/// Defines a new property that is closed under taking graph subclasses.
pub fn property(id: &str, name: &str, own: RawOwn, relevance: u32) -> SetBuilder {
    let res = BuiltRawSet::new(
        id.into(),
        name.into(),
        RawType::Property(own),
        Composition::None,
        relevance,
        );
    SetBuilder::new(res)
}

pub fn source(id: &str, sourcekey: &str, relevance: u32) -> RawDataSource {
    assert!(relevance <= 9);
    let rawsourcekey = RawSourceKey::Bibtex {
        key: sourcekey.into(),
    };
    let mut res = BuiltRawSource {
        id: Id::new(id.into()),
        rawsourcekey,
        relevance,
        drawings: vec![],
    };
    RawDataSource::new(res)
}

/// Tie to other websites that keep information about parameters.
pub fn provider(
    id: &str,
    name: &str,
    url: &str,
    format_url: Box<dyn Fn(&str) -> String>,
    ) -> RawDataProvider {
    let provider = RawProvider {
        id: ProviderId::new(id.into()),
        name: name.into(),
        url: url.into(),
    };
    RawDataProvider::new(provider, format_url)
}

impl Builder {
    pub fn new() -> Builder {
        let unknown_source = RawDataSource::new(BuiltRawSource {
            id: Id::new(UNKNOWN_SOURCE_ID.into()),
            rawsourcekey: RawSourceKey::Other {
                name: "unknown source".into(),
                description: "This knowledge was added to the database without tying it to an appropriate resource.".into(),
            },
            relevance: 3,
            drawings: vec![],
        });
        let assumed_source = RawDataSource::new(BuiltRawSource {
            id: Id::new(ASSUMED_SOURCE_ID.into()),
            rawsourcekey: RawSourceKey::Other {
                name: "assumed".into(),
                description: "Is axiomatic knowledge from the viewpoint of HOPS website.".into(),
            },
            relevance: 6,
            drawings: vec![],
        });
        let mut data = RawData::new();
        Self {
            data,
            id_sanity_map: HashSet::new(),
            name_sanity_map: HashSet::new(),
            unknown_source,
            assumed_source,
            set_map: HashMap::new(),
        }
    }

    fn get<'a>(&'a self, set: &PreviewSetId) -> &'a RawSet {
        let idx = self.set_map.get(set).unwrap();
        self.data.sets.get(*idx).unwrap()
    }

    /// Retrieves all the information that was added to the builder
    /// while making the builder unusable further.
    pub fn build(mut self) -> RawData {
        let Self {
            mut data,
            id_sanity_map: _,
            name_sanity_map: _,
            mut unknown_source,
            mut assumed_source,
            set_map: _,
        } = self;
        for set in &data.sets {
            assumed_source = assumed_source.proved(
                &ShowedId::get_tmp().to_string(),
                Page::NotApplicable,
                &set.id.preview(),
                &set.id.preview(),
                crate::general::enums::Cpx::Equal,
                "assumed",
                );
        }
        unknown_source.data_done(&mut data);
        assumed_source.data_done(&mut data);
        data
    }

    /// Adds set to the collection while making a few sanity checks.
    pub fn add_set(&mut self, set: RawSet) {
        if self.id_sanity_map.contains(&set.id.to_string()) {
            panic!("id {} used multiple times", set.id);
        }
        if set.id.to_string().len() != 6 {
            // todo - polish id sanity check
            error!("id {} has non-standard format", set.id);
        }
        if self.name_sanity_map.contains(&set.name) {
            error!("name {} used multiple times", set.name);
        }
        self.id_sanity_map.insert(set.id.to_string());
        self.name_sanity_map.insert(set.name.clone());
        let last_idx = self.data.sets.len();
        let set_id = set.id.preview();
        self.data.sets.push(set);
        self.set_map.insert(set_id, last_idx);
    }

    /// Represents that bounds on *from* instances transfer to *to* instances.
    /// For the bound to transfer both its endpoints must be from the same TranferGroup.
    pub fn transfers_bound_to(
        &mut self,
        group: TransferGroup,
        from: &PreviewSetId,
        to: &PreviewSetId,
    ) {
        let r = self.data.transfer.entry(group).or_default();
        r.push((from.clone(), to.clone()));
    }

    /// Add a parameter defined as the number of vertices to be removed
    /// until the remaining graph falls in the given set.
    pub fn distance_to(&self, id: &str, set_id: &PreviewSetId, relevance: u32) -> SetBuilder {
        let set = self.get(set_id);
        let res = BuiltRawSet::new(
            id.into(),
            format!("distance to {}", set.name),
            RawType::Parameter,
            Composition::None,
            relevance,
        );
        let set_type = set.typ.clone();
        let set_id = set.id.preview();
        SetBuilder::new(res)
            .displayed_definition("", &format!("Minimum number of vertices removed to make the graph into [[{}]]", set_id)) // todo move to later processing
            .add_callback(Box::new(
            move |builder: &mut Builder, newset: &RawSet| {
                let mut tmp_source = builder.assumed_source();
                match set_type {
                    RawType::Parameter => {
                        builder.assumed_source.ref_proved(
                            &SourceId::get_tmp().to_string(),
                            Page::NotApplicable,
                            &set_id,
                            &newset.id.preview(),
                            UpperBound(Linear),
                            "by definition",
                        )
                    },
                    RawType::GraphClass | RawType::Property(_) => {
                        builder.assumed_source.ref_proved(
                            &SourceId::get_tmp().to_string(),
                            Page::NotApplicable,
                            &set_id,
                            &newset.id.preview(),
                            UpperBound(Constant),
                            "by definition",
                        )
                    },
                };
                builder.transfers_bound_to(
                    TransferGroup::DistanceTo,
                    &set_id,
                    &newset.id.preview(),
                );
            },
        ))
    }

    /// Create a new set that represents intersection of sets.
    /// From a view point of classical parameterized complexity
    /// we may understand the intersection as a sum of parameters.
    pub fn intersection(
        &self,
        id: &str,
        set_a: &PreviewSetId,
        set_b: &PreviewSetId,
        name: &str,
        relevance: u32,
    ) -> SetBuilder {
        let sets = vec![set_a.clone(), set_b.clone()];
        let (typ, upper_bound) = if sets.iter().any(|x|{
            let set = self.get(x);
            set.typ == RawType::Parameter
        }) {
            (RawType::Parameter, UpperBound(Linear))
        } else {
            (RawType::GraphClass, UpperBound(Constant))
        };
        let res = BuiltRawSet::new(
            id.into(),
            name.into(),
            typ,
            Composition::Intersection(sets.clone()),
            relevance,
        );
        let mut definition = String::new();
        definition += "an intersetcion of";
        for (i, set_id) in sets.iter().enumerate() {
            let join = if i+1 == sets.len() {
                ", and"
            } else if i == 0 {
                " "
            } else if sets.len() == 2 {
                " "
            } else {
                ", "
            };
            definition += &format!("{} [[{}]]", join, set_id);
        }
        SetBuilder::new(res)
            .displayed_definition("", &definition)
            .add_callback(Box::new(
            move |builder: &mut Builder, newset: &RawSet| {
                for set_id in &sets {
                    let id = &format!("{}_{}", newset.id, set_id);
                    builder.assumed_source().ref_proved("", Page::NotApplicable, &newset.id.preview(), set_id, upper_bound.clone(), "by definition");
                }
            },
        ))
    }

    pub fn assumed_source(&mut self) -> &mut RawDataSource {
        &mut self.assumed_source
    }

    pub fn unknown_source(&mut self) -> &mut RawDataSource {
        &mut self.unknown_source
    }

    pub fn web_source(&mut self, id: &str, url: &str) -> RawDataSource {
        let rawsourcekey = RawSourceKey::Online { url: url.into() };
        let mut res = BuiltRawSource {
            id: Id::new(id.into()),
            rawsourcekey,
            relevance: 0,
            drawings: vec![],
        };
        RawDataSource::new(res)
    }

    /// Define a tag that some sets share so they can be grouped.
    pub fn tag(&mut self, id: &str, name: &str, description: &str) -> PreviewTagId {
        let res = RawTag {
            id: Id::new(id.into()),
            name: name.into(),
            description: description.into(),
        };
        let ret = res.id.preview();
        self.data.tags.push(res);
        ret
    }

    pub fn tag_set(&mut self, tag: PreviewTagId, set: PreviewSetId) {
        self.data.tag_set.push((tag, set));
    }
}
