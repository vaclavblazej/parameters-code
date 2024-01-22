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

    pub fn done(self) -> RawData {
        self.data
    }

    /// Represents that bounds on *from* instances transfer to *to* instances.
    /// For the bound to transfer both its endpoints must be from the same TranferGroup.
    pub fn transfers_bound_to(&mut self, group: TransferGroup, from: &RawSet, to: &RawSet) {
        // self.transfer[group];
        // .push((from, to));
    }

    pub fn parameter(&mut self, id: &str, name: &str) -> RawSet {
        let res = RawSet {
            id: id.into(),
            name: name.into(),
            kind: RawKind::Parameter
        };
        self.data.parameters.push(res.clone());
        res
    }

    pub fn distance_to(&mut self, set: &RawSet) -> RawSet {
        let res = RawSet {
            id: format!("distance_to_{}", set.id.clone()),
            name: set.name.clone(),
            kind: RawKind::Parameter
        };
        self.transfers_bound_to(TransferGroup::DistanceTo, &set, &res);
        res
    }

    pub fn graph_class(&mut self, id: &str, name: &str) -> RawSet {
        let res = RawSet {
            id: id.into(),
            name: name.into(),
            kind: RawKind::GraphClass
        };
        self.data.graph_classes.push(res.clone());
        res
    }

    pub fn connected(&mut self, set: &RawSet, name: &str) -> RawSet {
        let res = RawSet {
            id: format!("connected_{}", set.id),
            name: name.into(),
            kind: RawKind::GraphClass
        };
        self.data.graph_classes.push(res.clone());
        res
    }

    pub fn source(&mut self, id: &str, sourcekey: &str) -> DataSource {
        let rawsourcekey = if sourcekey.contains("://") {
            RawSourceKey::Online{ url: sourcekey.into() }
        } else {
            RawSourceKey::Bibtex{ key: sourcekey.into() }
        };
        let res = RawSource { id: id.into(), rawsourcekey, };
        self.data.sources.push(res.clone());
        DataSource::new(&res, &mut self.data)
    }

    pub fn isgci(&mut self, set: &RawSet, code: u32) -> &mut Self {
        self.data.isgci.push((set.clone(), code));
        self
    }

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
