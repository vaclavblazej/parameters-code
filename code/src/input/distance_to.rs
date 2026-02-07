use crate::data::data::NameCore;
use crate::data::id::*;
use crate::input::builder::Builder;
use crate::input::raw::RawParameter;
use crate::input::raw_enums::RawParameterDefinition;

pub trait DistanceTo<TargetId>
where
    Self: Sized,
{
    fn distance_to(id: &str, target: &TargetId, name: &str, score: u32) -> Builder<Self>;
}

impl DistanceTo<PreviewGraphClassId> for RawParameter {
    fn distance_to(
        id: &str,
        target: &PreviewGraphClassId,
        name: &str,
        score: u32,
    ) -> Builder<Self> {
        Builder::new(RawParameter {
            id: ParameterId::new(id),
            score,
            name_core: NameCore::new(name),
            definition: RawParameterDefinition::DistanceToGraphClass(target.clone()),
            tags: Vec::new(),
        })
    }
}

impl DistanceTo<PreviewParameterId> for RawParameter {
    fn distance_to(id: &str, target: &PreviewParameterId, name: &str, score: u32) -> Builder<Self> {
        Builder::new(RawParameter {
            id: ParameterId::new(id),
            score,
            name_core: NameCore::new(name),
            definition: RawParameterDefinition::DistanceToParameter(target.clone()),
            tags: Vec::new(),
        })
    }
}
