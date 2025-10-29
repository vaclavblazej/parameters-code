
use std::collections::HashMap;

use super::{
    raw::{
        BuiltRawSet, Composition, RawSet, RawType, RawSequence
    },
    set::SetBuilder,
    build::Builder,
};

pub enum ParameterType {
    Any,
    All,
    Value(u32),
    Infinity,
}

pub struct SequenceBuilder {
    id: String,
    map: HashMap<ParameterType,Option<SetBuilder>>,
}

impl SequenceBuilder {

    pub fn new(id: &str) -> Self {
        Self {
            id: String::from(id),
            map: HashMap::new(),
        }
    }

    pub fn parameter(&mut self, id: &str, name: &str, typ: ParameterType, relevance: u32) -> SetBuilder {
        let res = BuiltRawSet::new(
            id.into(),
            name.into(),
            RawType::Parameter,
            Composition::None,
            relevance,
        );
        SetBuilder::new(res)
            // .add_callback(Box::new(
                // move |builder: &mut Builder, newset: &RawSet| {
                    // todo!()
                // }))
    }

    pub fn done(self, builder: &mut Builder) {
        builder.data.sequences.push(RawSequence{
        });
    }

}
