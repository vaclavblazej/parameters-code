use crate::data::{data::NameCore, id::*};
use crate::input::raw::RawParameter;
use crate::input::raw_enums::{RawGraphClassDefinition, RawParameterDefinition};
use crate::input::{builder::Builder, raw::RawGraphClass};

pub trait Intersectable<Other> {
    type Result;
    fn intersect(id: &str, a: &Self, b: &Other, name: &str, score: u32) -> Builder<Self::Result>;
}

impl Intersectable<PreviewGraphClassId> for PreviewGraphClassId {
    type Result = RawGraphClass;
    fn intersect(
        id: &str,
        a: &Self,
        b: &PreviewGraphClassId,
        name: &str,
        score: u32,
    ) -> Builder<RawGraphClass> {
        Builder::new(RawGraphClass {
            id: GraphClassId::new(id),
            score,
            name_core: NameCore::new(name),
            definition: RawGraphClassDefinition::IntersectionGraphClasses(vec![
                a.clone(),
                b.clone(),
            ]),
            variant: crate::input::raw_enums::RawGraphClassVariant::GraphClass, // todo
            tags: Vec::new(),
        })
    }
}

impl Intersectable<PreviewGraphClassPropertyId> for PreviewParameterId {
    type Result = RawParameter;
    fn intersect(
        id: &str,
        a: &Self,
        b: &PreviewGraphClassPropertyId,
        name: &str,
        score: u32,
    ) -> Builder<RawParameter> {
        Builder::new(RawParameter {
            id: ParameterId::new(id),
            score,
            name_core: NameCore::new(name),
            definition: RawParameterDefinition::IntersectionParameterProperty(a.clone(), b.clone()),
            tags: Vec::new(),
        })
    }
}

impl Intersectable<PreviewGraphClassPropertyId> for PreviewGraphClassId {
    type Result = RawGraphClass;
    fn intersect(
        id: &str,
        a: &Self,
        b: &PreviewGraphClassPropertyId,
        name: &str,
        score: u32,
    ) -> Builder<RawGraphClass> {
        Builder::new(RawGraphClass {
            id: GraphClassId::new(id),
            score,
            name_core: NameCore::new(name),
            definition: RawGraphClassDefinition::IntersectionGraphClassProperty(
                a.clone(),
                b.clone(),
            ),
            variant: crate::input::raw_enums::RawGraphClassVariant::GraphClass,
            tags: Vec::new(),
        })
    }
}

impl Intersectable<PreviewGraphClassId> for PreviewParameterId {
    type Result = RawParameter;
    fn intersect(
        id: &str,
        a: &Self,
        b: &PreviewGraphClassId,
        name: &str,
        score: u32,
    ) -> Builder<RawParameter> {
        Builder::new(RawParameter {
            id: ParameterId::new(id),
            score,
            name_core: NameCore::new(name),
            definition: RawParameterDefinition::IntersectionParameterGraphClass(
                a.clone(),
                b.clone(),
            ),
            tags: Vec::new(),
        })
    }
}

impl Intersectable<PreviewParameterId> for PreviewParameterId {
    type Result = RawParameter;
    fn intersect(
        id: &str,
        a: &Self,
        b: &PreviewParameterId,
        name: &str,
        score: u32,
    ) -> Builder<RawParameter> {
        Builder::new(RawParameter {
            id: ParameterId::new(id),
            score,
            name_core: NameCore::new(name),
            definition: RawParameterDefinition::IntersectionParameters(vec![a.clone(), b.clone()]),
            tags: Vec::new(),
        })
    }
}
