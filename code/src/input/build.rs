//! Functions for creating the data collection.

use std::collections::{HashMap, HashSet};

use log::error;

use crate::data::data::GraphClass;
use crate::data::data::NameCore;
use crate::data::data::Named;
use crate::data::enums::*;
use crate::data::id::*;
use crate::input::builder::Builder;
use crate::input::intersectable::Intersectable;
use crate::input::provider::RawDataProvider;
use crate::input::raw::*;
use crate::input::raw_enums::*;
use crate::input::source::Cpx;
use crate::input::source::RawSource;
use crate::input::source::RawSourceData;
use crate::input::source::RawSourceKey;

pub fn copyvec<T>(vec: Vec<&PreviewId<T>>) -> Vec<PreviewId<T>>
where
    PreviewId<T>: Clone,
{
    vec.iter().map(|&x| x.clone()).collect()
}

pub struct CollectionBuilder {
    pub data: RawData,
    id_sanity_map: HashSet<String>, // todo move unique checking to id.rs?
    name_sanity_map: HashSet<String>,
    unknown_source: RawSourceData,
    assumed_source: RawSourceData,
}

pub static UNKNOWN_SOURCE_ID: &str = "myit4D";
pub static ASSUMED_SOURCE_ID: &str = "9kg0oo";

/// Defines a new graph class. We do not aim to have all graph
/// classes in the database but only those that are very relevant
/// to the field of parameterized complexity.
pub fn graph_class(id: &str, name: &str, score: u32, definition: &str) -> Builder<RawGraphClass> {
    assert!(score <= 9);
    Builder::new(RawGraphClass {
        id: GraphClassId::new(id.into()),
        score,
        name_core: NameCore::new(name),
        definition: RawGraphClassDefinition::Text(definition.into()),
        tags: Vec::new(),
        variant: RawGraphClassVariant::GraphClass,
    })
}

pub fn graph_property(
    id: &str,
    name: &str,
    score: u32,
    definition: &str,
) -> Builder<RawGraphClass> {
    assert!(score <= 9);
    Builder::new(RawGraphClass {
        id: GraphClassId::new(id.into()),
        score,
        name_core: NameCore::new(name),
        definition: RawGraphClassDefinition::Text(definition.into()),
        tags: Vec::new(),
        variant: RawGraphClassVariant::GraphProperty,
    })
}

/// Defines a graph class where each graph in it is associated with an integer.
pub fn parametric_graph_class(
    id: &str,
    name: &str,
    score: u32,
    closed_under: PreviewGraphRelationId,
    definition: &str,
) -> Builder<RawParametricGraphClass> {
    assert!(score <= 9);
    Builder::new(RawParametricGraphClass {
        id: ParametricGraphClassId::new(id.into()),
        score,
        name_core: NameCore::new(name),
        closed_under,
        tags: Vec::new(),
        definition: String::from(definition),
    })
}

/// Add a new parameter.
/// This typically represents a defined named parameter.
/// For ambiguous names we have no clear solution yet. (todo)
/// For parameters with more names each can be defined as
/// a separate parameter and then united with Equivalence.
/// Equivalent parameters whose equivalence is to some degree surprising
/// their definitions may be kept separate.
pub fn parameter(id: &str, name: &str, score: u32, definition: &str) -> Builder<RawParameter> {
    Builder::new(RawParameter {
        id: ParameterId::new(id.into()),
        score,
        name_core: NameCore::new(name),
        definition: RawParameterDefinition::GraphText(definition.into()),
        tags: Vec::new(),
    })
}

pub fn higher_order_parameter(
    id: &str,
    name: &str,
    score: u32,
    bounds_all: PreviewParametricParameterId,
) -> Builder<RawParameter> {
    Builder::new(RawParameter {
        id: ParameterId::new(id.into()),
        score,
        name_core: NameCore::new(name),
        definition: RawParameterDefinition::BoundsAll(bounds_all),
        tags: Vec::new(),
    })
}

pub fn parametric_parameter(
    id: &str,
    name: &str,
    score: u32,
    definition: RawParametricParameterDefinition,
) -> Builder<RawParametricParameter> {
    Builder::new(RawParametricParameter {
        id: ParametricParameterId::new(id.into()),
        score,
        name_core: NameCore::new(name),
        definition,
        tags: Vec::new(),
    })
}

pub fn graph_class_property(
    id: &str,
    name: &str,
    own: RawOwn,
    score: u32,
    definition: RawGraphClassPropertyDefinition,
) -> Builder<RawGraphClassProperty> {
    assert!(score <= 9);
    Builder::new(RawGraphClassProperty {
        id: GraphClassPropertyId::new(id.into()),
        score,
        name_core: NameCore::new(name),
        definition,
        own,
    })
}

pub fn source(id: &str, sourcekey: &str, score: u32) -> RawSourceData {
    assert!(score <= 9);
    let rawsourcekey = RawSourceKey::Bibtex {
        key: String::from(sourcekey),
    };
    let mut res = RawSource {
        id: Id::new(id.into()),
        rawsourcekey,
        score,
    };
    RawSourceData::new(res)
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
        name_core: NameCore::new(name),
        url: url.into(),
    };
    RawDataProvider::new(provider, format_url)
}

impl CollectionBuilder {
    pub fn new() -> CollectionBuilder {
        let unknown_source = RawSourceData::new(RawSource {
            id: Id::new(UNKNOWN_SOURCE_ID.into()),
            rawsourcekey: RawSourceKey::Other {
                name: "unknown source".into(),
                description: "This knowledge was added to the database without tying it to an appropriate resource.".into(),
            },
            score: 3,
        });
        let assumed_source = RawSourceData::new(RawSource {
            id: Id::new(ASSUMED_SOURCE_ID.into()),
            rawsourcekey: RawSourceKey::Other {
                name: "assumed".into(),
                description: "Is axiomatic knowledge from the viewpoint of HOPS website.".into(),
            },
            score: 6,
        });
        let mut data = RawData::new();
        Self {
            data,
            id_sanity_map: HashSet::new(),
            name_sanity_map: HashSet::new(),
            unknown_source,
            assumed_source,
        }
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
        } = self;
        // for set in &data.sets { // todo reflexivity
        //     assumed_source = assumed_source.proved(
        //         &ShowedId::get_tmp().to_string(),
        //         Page::NotApplicable,
        //         &set.id.preview(),
        //         &set.id.preview(),
        //         Cpx::Equal,
        //         "assumed",
        //     );
        // }
        unknown_source.data_done(&mut data);
        assumed_source.data_done(&mut data);
        data
    }

    /// Adds set to the collection while making a few sanity checks.
    pub fn add_set<T>(&mut self, set: T)
    where
        T: RawDataAddable + HasId + Named,
    {
        if self.id_sanity_map.contains(&set.id()) {
            panic!("id {} used multiple times", set.id());
        }
        if set.id().len() != 6 {
            // todo - polish id sanity check
            error!("id {} has non-standard format", set.id());
        }
        if self.name_sanity_map.contains(&set.name()) {
            error!("name {} used multiple times", set.name());
        }
        self.id_sanity_map.insert(set.id());
        self.name_sanity_map.insert(set.name());
        let set_id = set.id();
        set.add(&mut self.data);
    }

    // /// Represents that bounds on *from* instances transfer to *to* instances.
    // /// For the bound to transfer both its endpoints must be from the same TranferGroup.
    // pub fn transfers_bound_to( // todo
    //     &mut self,
    //     group: TransferGroup,
    //     from: &PreviewSetId,
    //     to: &PreviewSetId,
    // ) {
    //     let r = self.data.transfer.entry(group).or_default();
    //     r.push((from.clone(), to.clone()));
    // }

    pub fn logic_fragment(
        &self,
        id: &str,
        name: &str,
        description: Option<&str>,
    ) -> PreviewLogicFragmentId {
        let res = RawLogicFragment {
            id: LogicFragmentId::new(id.into()),
            name_core: NameCore::new(name),
            description: description.map(String::from),
        };
        res.id.preview()
    }

    pub fn graph_operation(&self, id: &str, name: &str, definition: &str) -> PreviewOperationId {
        let res = RawOperation {
            id: OperationId::new(id.into()),
            name_core: NameCore::new(name),
            definition: RawOperationDefinition::GraphOperation(definition.into()),
        };
        res.id.preview()
    }

    pub fn problem(
        &self,
        id: &str,
        name: &str,
        definition: RawProblemDefinition,
    ) -> PreviewProblemId {
        let id = ProblemId::new(id.into());
        id.preview()
    }

    pub fn graph_class_operation(
        &self,
        id: &str,
        name: &str,
        operation: RawOperationDefinition,
    ) -> PreviewOperationId {
        let res = RawOperation {
            id: OperationId::new(id.into()),
            name_core: NameCore::new(name),
            definition: operation,
        };
        res.id.preview()
    }

    pub fn graph_class_relation_type(
        &self,
        id: &str,
        name: &str,
        definition: RawGraphClassRelationDefinition,
    ) -> PreviewGraphClassRelationId {
        let res = RawGraphClassRelation {
            id: GraphClassRelationId::new(id.into()),
            name_core: NameCore::new(name),
            definition,
        };
        res.id.preview()
    }

    pub fn graph_relation_type(
        &self,
        id: &str,
        name: &str,
        displayed_definition: RawGraphRelationDefinition,
    ) -> PreviewGraphRelationId {
        let res = RawGraphRelation {
            id: GraphRelationId::new(id.into()),
            name_core: NameCore::new(name),
            displayed_definition,
        };
        res.id.preview()
    }

    // /// Add a parameter defined as the number of vertices to be removed
    // /// until the remaining graph falls in the given set.
    // pub fn distance_to(
    //     &self,
    //     id: &str,
    //     set_id: &PreviewParameterId, // todo generalize?
    //     score: u32,
    // ) -> Builder<RawParameter> {
    //     let set = self.data.parameters.get(set_id).unwrap();
    //     let res = RawParameter {
    //         id: ParameterId::new(id.into()),
    //         score,
    //         name_core: NameCore::new(&format!("distance to {}", set.name)),
    //         definition: RawParameterDefinition::DistanceTo(set_id.clone()),
    //         tags: Vec::new(),
    //     };
    //     let set_type = set.typ.clone();
    //     let set_id = set.id.preview();
    //     Builder::new(res)
    //     .displayed_definition(
    //         "",
    //         &format!(
    //             "Minimum number of vertices removed to make the graph into [[{}]]",
    //             set_id
    //         ),
    //     ) // todo move to later processing
    //     .add_callback(Box::new( // todo fixme
    //         move |builder: &mut CollectionBuilder, newset: &RawSet| {
    //             let mut tmp_source = builder.assumed_source();
    //             match set_type {
    //                 RawType::Parameter => builder.assumed_source.ref_proved(
    //                     &SourceId::get_tmp().to_string(),
    //                     Page::NotApplicable,
    //                     &set_id,
    //                     &newset.id.preview(),
    //                     UpperBound(Linear),
    //                     "by definition",
    //                 ),
    //                 RawType::GraphClass | RawType::Property(_) => {
    //                     builder.assumed_source.ref_proved(
    //                         &SourceId::get_tmp().to_string(),
    //                         Page::NotApplicable,
    //                         &set_id,
    //                         &newset.id.preview(),
    //                         UpperBound(Constant),
    //                         "by definition",
    //                     )
    //                 }
    //             };
    //             builder.transfers_bound_to(
    //                 TransferGroup::DistanceTo,
    //                 &set_id,
    //                 &newset.id.preview(),
    //             );
    //         },
    //     ))
    // }

    /// Create a new set that represents intersection of sets.
    /// From a view point of classical parameterized complexity
    /// we may understand the intersection as a sum of parameters.
    pub fn intersection<IdB, TypeA>(
        &self,
        id: &str,
        set_a: &TypeA::PreviewId,
        set_b: &IdB,
        name: &str,
        score: u32,
    ) -> Builder<TypeA>
    where
        TypeA: Intersectable<IdB> + HasPreviewId,
    {
        TypeA::intersect(id, set_a, set_b, name, score)
    }

    pub fn assumed_source(&mut self) -> &mut RawSourceData {
        &mut self.assumed_source
    }

    pub fn unknown_source(&mut self) -> &mut RawSourceData {
        &mut self.unknown_source
    }

    pub fn web_source(&mut self, id: &str, url: &str) -> RawSourceData {
        let rawsourcekey = RawSourceKey::Online { url: url.into() };
        let mut res = RawSource {
            id: SourceId::new(id.into()),
            rawsourcekey,
            score: 0,
        };
        RawSourceData::new(res)
    }

    /// Define a tag that some sets share so they can be grouped.
    pub fn tag(&mut self, id: &str, name: &str, description: &str) -> PreviewTagId {
        let res = RawTag {
            id: Id::new(id.into()),
            name_core: NameCore::new(name),
            description: description.into(),
        };
        let ret = res.id.preview();
        self.data.tags.push(res);
        ret
    }
}
