//! Functions for creating the data collection.

use std::collections::{HashMap, HashSet};

use log::error;

use crate::data::data::NameCore;
use crate::data::enums::*;
use crate::data::id::*;
use crate::general::strings::nice_concat;
use crate::input::builder::Builder;
use crate::input::raw::*;
use crate::input::raw_enums::*;
use crate::input::source::RawSourceData;

pub fn copyvec<T>(vec: Vec<&PreviewId<T>>) -> Vec<PreviewId<T>> {
    vec.iter().map(|&x| x.to_owned()).collect()
}

pub struct CollectionBuilder {
    pub data: RawData,
    id_sanity_map: HashSet<String>, // todo move unique checking to id.rs?
    name_sanity_map: HashSet<String>,
    unknown_source: RawSourceData,
    assumed_source: RawSourceData,
    id_map: HashMap<String, usize>, // idx to data.sets
}

pub static UNKNOWN_SOURCE_ID: &str = "myit4D";
pub static ASSUMED_SOURCE_ID: &str = "9kg0oo";

/// Defines a new graph class. We do not aim to have all graph
/// classes in the database but only those that are very relevant
/// to the field of parameterized complexity.
pub fn graph_class(
    id: &str,
    name: &str,
    relevance: u32,
    definition: &str,
) -> Builder<PreviewGraphClassId, RawGraphClass> {
    assert!(relevance <= 9);
    Builder::new(RawGraphClass {
        id: GraphClassId::new(id),
        relevance,
        names: NameCore::new(name),
        definition: RawGraphClassDefinition::Text(definition),
    })
}

/// Defines a graph class where each graph in it is associated with an integer.
pub fn parametric_graph_class(
    id: &str,
    name: &str,
    relevance: u32,
    closed_under: PreviewGraphRelationId,
    definition: &str,
) -> Builder<PreviewParametricGraphClassId, RawParametricGraphClass> {
    assert!(relevance <= 9);
    Builder::new(RawParametricGraphClass {
        id: ParametricGraphClassId::new(id),
        relevance,
        names: NameCore::new(name),
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
pub fn parameter(
    id: &str,
    name: &str,
    relevance: u32,
    definition: &str,
) -> Builder<PreviewParameterId, RawParameter> {
    Builder::new(RawParameter {
        id: ParameterId::from(id.into()),
        relevance,
        names: NameCore::new(name),
        definition: RawParameterDefinition::text(String::from(definition)),
        tags: Vec::new(),
    })
}

pub fn higher_order_parameter(
    id: &str,
    name: &str,
    relevance: u32,
    bounds_all: PreviewParametricParameterId,
) -> Builder<PreviewHigherOrderParameterId, RawParameter> {
    Builder::new(RawParameter {
        id: HigherOrderParameterId::new(id.into()),
        relevance,
        names: NameCore::new(name),
        definition: RawParameterDefinition::BoundsAll(bounds_all),
        tags: Vec::new(),
    })
}

pub fn parametric_parameter(
    id: &str,
    name: &str,
    relevance: u32,
    definition: RawParametricParameterDefinition,
) -> Builder<PreviewParametricParameterId, RawParametricParameter> {
    Builder::new(RawParametricParameter {
        id: ParametricParameterId::new(id.into()),
        relevance,
        names: NameCore::new(name),
        definition,
        tags: Vec::new(),
    })
}

pub fn graph_property(
    id: &str,
    name: &str,
    own: RawOwn,
    relevance: u32,
    definition: RawGraphProperty,
) -> Builder<PreviewGraphClassPropertyId, RawGraphClassProperty> {
    assert!(relevance <= 9);
    Builder::new(RawGraphClass {
        id: GraphClassPropertyId::from(id.into()),
        relevance,
        names: NameCore::new(name),
        definition,
        own,
    })
}

pub fn graph_class_property(
    id: &str,
    name: &str,
    own: RawOwn,
    relevance: u32,
    definition: RawGraphClassPropertyDefinition,
) -> Builder<PreviewGraphClassPropertyId, RawGraphClassProperty> {
    assert!(relevance <= 9);
    Builder::new(RawGraphClassProperty {
        id: GraphClassPropertyId::new(id.into()),
        relevance,
        names: NameCore::new(name),
        definition,
        own,
    })
}

pub fn source(id: &str, sourcekey: &str, relevance: u32) -> RawSourceData {
    assert!(relevance <= 9);
    let rawsourcekey = RawSourceKey::Bibtex {
        key: String::from(sourcekey),
    };
    let mut res = BuiltRawSource {
        id: Id::new(id.into()),
        rawsourcekey,
        relevance,
        drawings: vec![],
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
        name: name.into(),
        url: url.into(),
    };
    RawDataProvider::new(provider, format_url)
}

impl CollectionBuilder {
    pub fn new() -> CollectionBuilder {
        let unknown_source = RawSourceData::new(BuiltRawSource {
            id: Id::new(UNKNOWN_SOURCE_ID.into()),
            rawsourcekey: RawSourceKey::Other {
                name: "unknown source".into(),
                description: "This knowledge was added to the database without tying it to an appropriate resource.".into(),
            },
            relevance: 3,
            drawings: vec![],
        });
        let assumed_source = RawSourceData::new(BuiltRawSource {
            id: Id::new(ASSUMED_SOURCE_ID.into()),
            rawsourcekey: RawSourceKey::Other {
                name: "assumed".into(),
                description: "Is axiomatic knowledge from the viewpoint of HOPS website.".into(),
            },
            relevance: 6,
            drawings: vec![],
        });
        let mut data = RawData::new();
        Self {
            data,
            id_sanity_map: HashSet::new(),
            name_sanity_map: HashSet::new(),
            unknown_source,
            assumed_source,
            id_map: HashMap::new(),
        }
    }

    fn get<'a, T, W>(&'a self, set: &W) -> &'a T
    where
        T: HasId<W>,
    {
        let idx = self.id_map.get(set).unwrap();
        self.data.sets.get(*idx).unwrap()
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
            id_map: _,
        } = self;
        for set in &data.sets {
            assumed_source = assumed_source.proved(
                &ShowedId::get_tmp().to_string(),
                Page::NotApplicable,
                &set.id.preview(),
                &set.id.preview(),
                crate::data::enums::Cpx::Equal,
                "assumed",
            );
        }
        unknown_source.data_done(&mut data);
        assumed_source.data_done(&mut data);
        data
    }

    /// Adds set to the collection while making a few sanity checks.
    pub fn add_set(&mut self, set: RawSet) {
        if self.id_sanity_map.contains(&set.id.to_string()) {
            panic!("id {} used multiple times", set.id);
        }
        if set.id.to_string().len() != 6 {
            // todo - polish id sanity check
            error!("id {} has non-standard format", set.id);
        }
        if self.name_sanity_map.contains(&set.name) {
            error!("name {} used multiple times", set.name);
        }
        self.id_sanity_map.insert(set.id.to_string());
        self.name_sanity_map.insert(set.name.clone());
        let last_idx = self.data.sets.len();
        let set_id = set.id.preview();
        self.data.sets.push(set);
        self.id_map.insert(set_id, last_idx);
    }

    /// Represents that bounds on *from* instances transfer to *to* instances.
    /// For the bound to transfer both its endpoints must be from the same TranferGroup.
    pub fn transfers_bound_to(
        &mut self,
        group: TransferGroup,
        from: &PreviewSetId,
        to: &PreviewSetId,
    ) {
        let r = self.data.transfer.entry(group).or_default();
        r.push((from.clone(), to.clone()));
    }

    pub fn logic_fragment(
        &self,
        id: &str,
        name: &str,
        description: Option<&str>,
    ) -> PreviewLogicFragmentId {
        let res = RawLogicFragment {
            id: LogicFragmentId::new(id.into()),
            name: String::from(name),
            description: description.map(String::from),
        };
        res.id.preview()
    }

    pub fn graph_operation(
        &self,
        id: &str,
        name: &str,
        definition: RawOperationDefinition,
    ) -> PreviewGraphOperationId {
        let res = RawOperation {
            id: GraphOperationId::new(id.into()),
            name: NameCore::new(name),
            definition,
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
        definition: GraphClassOperationDefinition,
    ) -> PreviewGraphClassOperationId {
        let res = RawGraphClassOperation {
            id: GraphClassOperationId::new(id.into()),
            name: String::from(name),
            definition,
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
            name: String::from(name),
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
            name: String::from(name),
            displayed_definition,
        };
        res.id.preview()
    }

    /// Add a parameter defined as the number of vertices to be removed
    /// until the remaining graph falls in the given set.
    pub fn distance_to(&self, id: &str, set_id: &PreviewSetId, relevance: u32) -> SetBuilder {
        let set = self.get(set_id);
        let res = BuiltRawSet::new(
            id.into(),
            format!("distance to {}", set.name),
            RawType::Parameter,
            Composition::None,
            relevance,
        );
        let set_type = set.typ.clone();
        let set_id = set.id.preview();
        SetBuilder::new(res)
            .displayed_definition(
                "",
                &format!(
                    "Minimum number of vertices removed to make the graph into [[{}]]",
                    set_id
                ),
            ) // todo move to later processing
            .add_callback(Box::new(
                move |builder: &mut CollectionBuilder, newset: &RawSet| {
                    let mut tmp_source = builder.assumed_source();
                    match set_type {
                        RawType::Parameter => builder.assumed_source.ref_proved(
                            &SourceId::get_tmp().to_string(),
                            Page::NotApplicable,
                            &set_id,
                            &newset.id.preview(),
                            UpperBound(Linear),
                            "by definition",
                        ),
                        RawType::GraphClass | RawType::Property(_) => {
                            builder.assumed_source.ref_proved(
                                &SourceId::get_tmp().to_string(),
                                Page::NotApplicable,
                                &set_id,
                                &newset.id.preview(),
                                UpperBound(Constant),
                                "by definition",
                            )
                        }
                    };
                    builder.transfers_bound_to(
                        TransferGroup::DistanceTo,
                        &set_id,
                        &newset.id.preview(),
                    );
                },
            ))
    }

    /// Create a new set that represents intersection of sets.
    /// From a view point of classical parameterized complexity
    /// we may understand the intersection as a sum of parameters.
    pub fn intersection(
        &self,
        id: &str,
        set_a: &PreviewSetId,
        set_b: &PreviewSetId,
        name: &str,
        relevance: u32,
    ) -> SetBuilder {
        let sets = vec![set_a.clone(), set_b.clone()];
        let (typ, upper_bound) = if sets.iter().any(|x| {
            let set = self.get(x);
            set.typ == RawType::Parameter
        }) {
            (RawType::Parameter, UpperBound(Linear))
        } else {
            (RawType::GraphClass, UpperBound(Constant))
        };
        let res = BuiltRawSet::new(
            id.into(),
            name.into(),
            typ,
            Composition::Intersection(sets.clone()),
            relevance,
        );
        let definition = format!("an intersetcion of {}", nice_concat(sets));
        SetBuilder::new(res)
            .displayed_definition("", &definition)
            .add_callback(Box::new(
                move |builder: &mut CollectionBuilder, newset: &RawSet| {
                    for set_id in &sets {
                        let id = &format!("{}_{}", newset.id, set_id);
                        builder.assumed_source().ref_proved(
                            "",
                            Page::NotApplicable,
                            &newset.id.preview(),
                            set_id,
                            upper_bound.clone(),
                            "by definition",
                        );
                    }
                },
            ))
    }

    pub fn assumed_source(&mut self) -> &mut RawSourceData {
        &mut self.assumed_source
    }

    pub fn unknown_source(&mut self) -> &mut RawSourceData {
        &mut self.unknown_source
    }

    pub fn web_source(&mut self, id: &str, url: &str) -> RawSourceData {
        let rawsourcekey = RawSourceKey::Online { url: url.into() };
        let mut res = BuiltRawSource {
            id: SourceId::new(id.into()),
            rawsourcekey,
            relevance: 0,
        };
        RawSourceData::new(res)
    }

    /// Define a tag that some sets share so they can be grouped.
    pub fn tag(&mut self, id: &str, name: &str, description: &str) -> PreviewTagId {
        let res = RawTag {
            id: Id::new(id.into()),
            name: name.into(),
            description: description.into(),
        };
        let ret = res.id.preview();
        self.data.tags.push(res);
        ret
    }

    pub fn tag_set(&mut self, tag: PreviewTagId, set: PreviewSetId) {
        self.data.tag_set.push((tag, set));
    }
}
