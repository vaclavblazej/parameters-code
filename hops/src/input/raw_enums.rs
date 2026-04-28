use serde::{Deserialize, Serialize};

use crate::data::id::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum RawOwn {
    Has,
    Is,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RawParameterDefinition {
    GraphText(String),
    GraphClassText(String),
    BoundsAll(PreviewParametricParameterId),
    DistanceToParameter(PreviewParameterId),
    DistanceToGraphClass(PreviewGraphClassId),
    IntersectionParameters(Vec<PreviewParameterId>),
    IntersectionParameterProperty(PreviewParameterId, PreviewGraphClassPropertyId),
    IntersectionParameterGraphClass(PreviewParameterId, PreviewGraphClassId),
    FromParametricParameter(PreviewParametricParameterId),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RawOperationDefinition {
    GraphOperation(PreviewOperationId),
    GraphClassOperation(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RawGraphClassPropertyDefinition {
    Text(String),
    FromGraphClass(PreviewGraphClassId),
    FromParameter(PreviewParameterId),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RawProblemDefinition {
    Text(String),
    ModelChecking(PreviewLogicFragmentId),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RawGraphRelationDefinition {
    Text(String),
    IsomorphicAfterOperations(Vec<PreviewOperationId>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RawGraphClassRelationDefinition {
    Text(String),
    GraphRelation(PreviewGraphRelationId),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RawGraphClassDefinition {
    Text(String),
    IntersectionGraphClasses(Vec<PreviewGraphClassId>),
    IntersectionGraphClassProperty(PreviewGraphClassId, PreviewGraphClassPropertyId),
    ParametricGraphClass(PreviewParametricGraphClassId),
    Parameter(PreviewParameterId),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RawGraphClassVariant {
    GraphClass,
    GraphProperty,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RawParametricParameterDefinition {
    GraphClassParameter(String),
    GraphParameter(String),
}
