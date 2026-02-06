use crate::data::data::ParameterDefinition;
use crate::data::{data::NameCore, id::*};
use crate::input::raw::RawParameter;
use crate::input::raw_enums::{RawGraphClassDefinition, RawParameterDefinition};
use crate::input::{builder::Builder, raw::RawGraphClass};

pub trait Intersectable<FirstId, SecondId>
where
    Self: Sized,
{
    fn intersect(id: &str, a: &FirstId, b: &SecondId, name: &str, score: u32) -> Builder<Self>;
}

impl Intersectable<PreviewGraphClassId, PreviewGraphClassId> for RawGraphClass {
    fn intersect(
        id: &str,
        a: &PreviewGraphClassId,
        b: &PreviewGraphClassId,
        name: &str,
        score: u32,
    ) -> Builder<Self> {
        Builder::new(RawGraphClass {
            id: GraphClassId::new(id),
            score,
            name_core: NameCore::new(name),
            definition: crate::input::raw_enums::RawGraphClassDefinition::Intersection(vec![
                a.clone(),
                b.clone(),
            ]),
            variant: crate::input::raw_enums::RawGraphClassVariant::GraphClass, // todo
            tags: Vec::new(),
        })
        // .add_callback(Box::new(
        //     move |builder: &mut CollectionBuilder, newset: &RawGraphClass| {
        //         for set_id in &vec![a, b] {
        //             let id = &format!("{}_{}", newset.id, set_id);
        //             // .wrote(Pp(2), "by definition", vec![("", todost, definition(&carving_width))])
        //             // builder.assumed_source().ref_proved(
        //             //     id,
        //             //     Page::NotApplicable,
        //             //     &newset.id.preview(),
        //             //     set_id,
        //                 , // todo fixme
        //             // );
        //         }
        //     },
        // ))
    }
}

// impl Intersectable<PreviewGraphClassId, PreviewGraphClassPropertyId> for RawGraphClass {
//     fn intersect(
//         id: &str,
//         a: &PreviewGraphClassId,
//         b: &PreviewGraphClassPropertyId,
//         name: &str,
//         score: u32,
//     ) -> Builder<Self> {
//         Builder::new(RawGraphClass {
//             id: GraphClassId::new(id),
//             score,
//             name_core: NameCore::new(name),
//             definition: RawGraphClassDefinition::Intersection(vec![a.clone(), b.clone()]),
//             variant: crate::input::raw_enums::RawGraphClassVariant::GraphClass,
//             tags: Vec::new(),
//         })
//     }
// }

// impl Intersectable<PreviewParameterId, PreviewGraphClassId> for RawParameter {
//     fn intersect(
//         id: &str,
//         a: &PreviewParameterId,
//         b: &PreviewGraphClassId,
//         name: &str,
//         score: u32,
//     ) -> Builder<Self> {
//         Builder::new(RawParameter {
//             id: ParameterId::new(id),
//             score,
//             name_core: NameCore::new(name),
//             definition: RawParameterDefinition::Intersection(vec![a.clone(), b.clone()]),
//             tags: Vec::new(),
//         })
//     }
// }

impl Intersectable<PreviewParameterId, PreviewParameterId> for RawParameter {
    fn intersect(
        id: &str,
        a: &PreviewParameterId,
        b: &PreviewParameterId,
        name: &str,
        score: u32,
    ) -> Builder<Self> {
        Builder::new(RawParameter {
            id: ParameterId::new(id),
            score,
            name_core: NameCore::new(name),
            definition: RawParameterDefinition::Intersection(vec![a.clone(), b.clone()]),
            tags: Vec::new(),
        })
    }
}
