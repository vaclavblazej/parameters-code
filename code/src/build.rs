//! Interface for a simplified input of connection information.

use std::collections::HashMap;

use crate::{data::{TransferGroup, DataSource, RawData}, raw::RawSourceKey};
use super::raw::{RawSet, RawKind, RawTopic, RawSource};

pub struct Builder {
    data: RawData,
}

impl Builder {

    pub fn new() -> Builder {
        Builder{
            data: RawData {
                parameters: Vec::new(),
                factoids: Vec::new(),
                graph_classes: Vec::new(),
                sources: Vec::new(),
                isgci: Vec::new(),
                topics: Vec::new(),
                transfer: HashMap::new(),
            }
        }
    }

    /// Retrieves all the information that was added to the builder
    /// while making the builder unusable further.
    pub fn build(self) -> RawData {
        self.data
    }

    /// Represents that bounds on *from* instances transfer to *to* instances.
    /// For the bound to transfer both its endpoints must be from the same TranferGroup.
    pub fn transfers_bound_to(&mut self, group: TransferGroup, from: &RawSet, to: &RawSet) {
        let r = self.data.transfer.entry(group).or_insert_with(Vec::new);
        r.push((from.clone(), to.clone()));
    }

    /// Add a new parameter.
    /// This typically represents a defined named parameter.
    /// For ambiguous names we have no clear solution yet.
    /// For parameters with more names each can be defined as
    /// a separate parameter and then united with Equality.
    /// Equivalent parameters whose equivalence is to a degree surprising
    /// their definitions may be kept separate.
    pub fn parameter(&mut self, id: &str, name: &str) -> RawSet {
        let res = RawSet {
            id: id.into(),
            name: name.into(),
            kind: RawKind::Parameter
        };
        self.data.parameters.push(res.clone());
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
        self.data.parameters.push(res.clone());
        self.transfers_bound_to(TransferGroup::DistanceTo, &set, &res);
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
        self.data.graph_classes.push(res.clone());
        res
    }

    /// Given a set, define a new graph class that requries that the
    /// graph is additionally connected. E.g. one can define a graph
    /// class of forests and then define trees as forest that is
    /// additionally connected.
    pub fn connected(&mut self, set: &RawSet, name: &str) -> RawSet {
        let res = RawSet {
            id: format!("connected_{}", set.id),
            name: name.into(),
            kind: RawKind::GraphClass
        };
        self.data.graph_classes.push(res.clone());
        res
    }

    /// Define a source of information. This includes online sources
    /// or reserach paper sources.
    pub fn source(&mut self, id: &str, sourcekey: &str) -> DataSource {
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
