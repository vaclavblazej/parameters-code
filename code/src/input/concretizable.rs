use crate::data::{data::NameCore, enums::Value, id::*};
use crate::input::builder::Builder;
use crate::input::raw::RawParameter;
use crate::input::raw_enums::RawParameterDefinition;

pub trait Concretizable {
    type Result;
    fn concretize(id: &str, a: &Self, value: Value, name: &str, score: u32) -> Builder<Self::Result>;
}

impl Concretizable for PreviewParametricParameterId {
    type Result = RawParameter;
    fn concretize(
        id: &str,
        a: &Self,
        value: Value,
        name: &str,
        score: u32,
    ) -> Builder<RawParameter> {
        Builder::new(RawParameter {
            id: ParameterId::new(id),
            score,
            name_core: NameCore::new(name),
            definition: RawParameterDefinition::FromParametricParameter(a.clone()),
            tags: Vec::new(),
        })
    }
}
