//! Interface for a simplified input of connection information.

use std::collections::{HashMap, HashSet};

use crate::general::enums::{Page, TransferGroup, CpxTime::{Linear, Constant}, Cpx::UpperBound};
use super::raw::{Composition, RawData, RawKind, RawSet, RawSource, RawSourceKey, RawTopic};
use super::source::RawDataSource;

pub struct Builder {
    data: RawData,
    id_sanity_map: HashSet<String>,
    name_sanity_map: HashSet<String>,
    unknown: RawSource,
    assumed_knowledge: RawSource,
}

impl Builder {

    pub fn new() -> Builder {
        let unknown_source = RawSource {
            id: "myit4D".into(),
            rawsourcekey: RawSourceKey::Other {
                name: "unknown".into(),
                description: "This knowledge was added to the database without tying it to an appropriate resource.".into(),
            },
        };
        let assumed_knowledge_source = RawSource {
            id: "9kg0oo".into(),
            rawsourcekey: RawSourceKey::Other {
                name: "assumed".into(),
                description: "Is axiomatic knowledge from the viewpoint of HOPS website.".into(),
            },
        };
        let mut data = RawData::new();
        data.sources.push(unknown_source.clone());
        Self {
            data,
            id_sanity_map: HashSet::new(),
            name_sanity_map: HashSet::new(),
            unknown: unknown_source,
            assumed_knowledge: assumed_knowledge_source,
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
            println!("name {} used multiple times", set.name);
        }
        self.data.sets.push(set.clone());
        if set.id != "" { // todo get rid of this exception; maybe recommend what ID could be used
                          // for this structure in eprintln
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
    /// Equivalent parameters whose equivalence is to some degree surprising
    /// their definitions may be kept separate.
    pub fn parameter(&mut self, id: &str, name: &str, popularity: u32) -> RawSet {
        let res = RawSet {
            id: id.into(),
            name: name.into(),
            kind: RawKind::Parameter,
            composed: None,
            popularity,
            hidden: false,
        };
        self.add_set(&res);
        res
    }

    /// Add a parameter defined as bounded function of the red degree created
    /// via a contraction sequence.
    pub fn reduced(&mut self, name: &str, set: &RawSet, popularity: u32) -> RawSet {
        let res = RawSet {
            id: format!("reduced_{}", set.id.clone()),
            name: name.to_string(),
            kind: RawKind::Parameter,
            composed: None,
            popularity,
            hidden: false,
        };
        self.add_set(&res);
        res
    }

    /// Add a parameter defined as the number of vertices to be removed
    /// until the remaining graph falls in the given set.
    pub fn distance_to(&mut self, set: &RawSet, popularity: u32) -> RawSet {
        let res = RawSet {
            id: format!("distance_to_{}", set.id.clone()),
            name: format!("distance to {}", set.name.clone()),
            kind: RawKind::Parameter,
            composed: None,
            popularity,
            hidden: false,
        };
        self.add_set(&res);
        let mut tmp_source = self.unknown_source();
        match set.kind {
            RawKind::Parameter => tmp_source.showed("", Page::NotApplicable, &set, &res, UpperBound(Linear), "by definition"),
            RawKind::GraphClass => tmp_source.showed("", Page::NotApplicable, &set, &res, UpperBound(Constant), "by definition"),
        };
        self.transfers_bound_to(TransferGroup::DistanceTo, &set, &res);
        res
    }

    /// Add a parameter defined as the minimum number of graphs from another set required
    /// to cover the edges of the input graph.
    pub fn edge_cover_by(&mut self, set: &RawSet) -> RawSet {
        let res = RawSet {
            id: format!("edge_cover_by_{}", set.id.clone()),
            name: format!("edge cover by {}", set.name.clone()),
            kind: RawKind::Parameter,
            composed: None,
            popularity: set.popularity,
            hidden: false,
        };
        self.add_set(&res);
        let mut tmp_source = self.unknown_source();
        tmp_source.showed("", Page::NotApplicable, &set, &res, UpperBound(Constant), "by definition");
        self.transfers_bound_to(TransferGroup::EdgeCover, &set, &res);
        res
    }

    /// Create a new set that represents intersection of sets.
    /// From a view point of classical parameterized complexity
    /// we may understand the intersection as a sum of parameters.
    pub fn intersection(&mut self, id: &str, set_a: &RawSet, set_b: &RawSet, name: &str) -> RawSet {
        let sets = vec![set_a.clone(), set_b.clone()];
        let (kind, upper_bound) = if sets.iter().all(|x|x.kind == RawKind::GraphClass) {
            (RawKind::GraphClass, UpperBound(Constant))
        } else {
            (RawKind::Parameter, UpperBound(Linear))
        };
        let res = RawSet {
            id: id.into(),
            name: name.into(),
            kind,
            composed: Some(Composition::Intersection(sets.clone())),
            popularity: 0, // todo
            hidden: false,
        };
        self.add_set(&res);
        // todo polish how these structures are created this; perhaps
        // add a global source that holds all things that are known by definition
        let mut tmp_source = self.unknown_source();
        for s in &sets {
            tmp_source = tmp_source.showed("", Page::NotApplicable, &res, &s, upper_bound.clone(), "by definition");
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
            kind: RawKind::GraphClass,
            composed: None,
            popularity: 0, // todo
            hidden: false,
        };
        self.add_set(&res);
        res
    }

    pub fn assumed_source(&mut self) -> RawDataSource {
        self.data.sources.push(self.assumed_knowledge.clone());
        RawDataSource::new(&self.assumed_knowledge, &mut self.data)
    }

    pub fn unknown_source(&mut self) -> RawDataSource {
        RawDataSource::new(&self.unknown, &mut self.data)
    }

    pub fn source(&mut self, id: &str, sourcekey: &str) -> RawDataSource {
        let rawsourcekey = RawSourceKey::Bibtex{ key: sourcekey.into() };
        let res = RawSource { id: id.into(), rawsourcekey, };
        self.data.sources.push(res.clone());
        RawDataSource::new(&res, &mut self.data)
    }

    pub fn web_source(&mut self, id: &str, url: &str) -> RawDataSource {
        let rawsourcekey = RawSourceKey::Online{ url: url.into() };
        let res = RawSource { id: id.into(), rawsourcekey, };
        self.data.sources.push(res.clone());
        RawDataSource::new(&res, &mut self.data)
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
