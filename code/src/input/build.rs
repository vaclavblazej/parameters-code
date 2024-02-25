//! Interface for a simplified input of connection information.

use std::collections::{HashMap, HashSet};

use crate::{data::data::{TransferGroup, DataSource}, input::raw::{RawData, RawSourceKey}};
use crate::data::data::Cpx::UpperBound;
use super::raw::{RawSet, RawKind, RawTopic, RawSource};
use crate::data::data::Page::Unknown;
use crate::complexity::time::CpxTime::Linear;

pub struct Builder {
    data: RawData,
    id_sanity_map: HashSet<String>,
    name_sanity_map: HashSet<String>,
}

impl Builder {

    pub fn new() -> Builder {
        Self {
            data: RawData::new(),
            id_sanity_map: HashSet::new(),
            name_sanity_map: HashSet::new(),
        }
    }

    /// Retrieves all the information that was added to the builder
    /// while making the builder unusable further.
    pub fn build(self) -> RawData {
        self.data
    }

    /// Adds set to the collection while making a few sanity checks.
    fn add_set(&mut self, set: &RawSet) {
        if self.id_sanity_map.contains(&set.id) {
            panic!("id {} used multiple times", set.id);
        }
        if self.name_sanity_map.contains(&set.name) {
            panic!("name {} used multiple times", set.name);
        }
        self.data.sets.push(set.clone());
        if set.id != "" { // todo get rid of this exception
            self.id_sanity_map.insert(set.id.clone());
            self.name_sanity_map.insert(set.name.clone());
        }
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
    /// a separate parameter and then united with Equavilence.
    /// Equivalent parameters whose equivalence is to a degree surprising
    /// their definitions may be kept separate.
    pub fn parameter(&mut self, id: &str, name: &str) -> RawSet {
        let res = RawSet {
            id: id.into(),
            name: name.into(),
            kind: RawKind::Parameter
        };
        self.add_set(&res);
        res
    }

    /// Add a parameter defined as the number of vertices to be removed
    /// until the remaining graph falls in the given set.
    pub fn distance_to(&mut self, set: &RawSet) -> RawSet {
        let res = RawSet {
            id: format!("distance_to_{}", set.id.clone()),
            name: format!("distance to {}", set.name.clone()),
            kind: RawKind::Parameter
        };
        self.add_set(&res);
        self.transfers_bound_to(TransferGroup::DistanceTo, &set, &res);
        res
    }

    /// Create a new set that represents intersection of sets.
    /// From a view point of classical parameterized complexity
    /// we may understand the intersection as a sum of parameters.
    pub fn intersection(&mut self, id: &str, set_a: &RawSet, set_b: &RawSet, name: &str) -> RawSet {
        let sets = vec![set_a.clone(), set_b.clone()];
        let res = RawSet {
            id: id.into(),
            name: name.into(),
            kind: RawKind::Intersection(sets.clone())
        };
        self.add_set(&res);
        // todo polish how these structures are created this; perhaps
        // add a global source that holds all things that are known by definition
        let mut tmp_source = self.source("", "unknown");
        for s in &sets {
            tmp_source = tmp_source.showed("", Unknown, &res, &s, UpperBound(Linear), "by definition");
        }
        res
    }

    /// Defines a new graph class. We do not aim to have all graph
    /// classes in the database but only those that are very relevant
    /// to the field of parameterized complexity.
    pub fn graph_class(&mut self, id: &str, name: &str) -> RawSet {
        let res = RawSet {
            id: id.into(),
            name: name.into(),
            kind: RawKind::GraphClass
        };
        self.add_set(&res);
        res
    }

    /// Define a source of information. This includes online sources
    /// or reserach paper sources.
    pub fn source(&mut self, id: &str, sourcekey: &str) -> DataSource {
        // todo improve this
        let rawsourcekey = if sourcekey.contains("://") {
            RawSourceKey::Online{ url: sourcekey.into() }
        } else if sourcekey == "unknown" {
            RawSourceKey::Unknown
        } else {
            RawSourceKey::Bibtex{ key: sourcekey.into() }
        };
        let res = RawSource { id: id.into(), rawsourcekey, };
        self.data.sources.push(res.clone());
        DataSource::new(&res, &mut self.data)
    }

    /// Tie an identifier from the "Information System on Graph Classes
    /// and their Inclusions" with the defined graph classes or parameters.
    pub fn isgci(&mut self, set: &RawSet, code: u32) -> &mut Self {
        self.data.isgci.push((set.clone(), code));
        self
    }

    /// Define a topic or property that some parameters share so they
    /// can be listed by them.
    pub fn topic(&mut self, id: &str, name: &str, description: &str) -> RawTopic {
        let res = RawTopic {
            id: id.into(),
            name: name.into(),
            description: description.into(),
        };
        self.data.topics.push(res.clone());
        res
    }

}
