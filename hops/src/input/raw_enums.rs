use crate::data::id::*;

#[derive(Debug, PartialEq, Clone)]
pub enum RawOwn {
    Has,
    Is,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum RawOperationDefinition {
    GraphOperation(PreviewOperationId),
    GraphClassOperation(String),
}

#[derive(Debug)]
pub enum RawGraphClassPropertyDefinition {
    Text(String),
    FromGraphClass(PreviewGraphClassId),
    FromParameter(PreviewParameterId),
}

#[derive(Debug)]
pub enum RawProblemDefinition {
    Text(String),
    ModelChecking(PreviewLogicFragmentId),
}

#[derive(Debug)]
pub enum RawGraphRelationDefinition {
    Text(String),
    IsomorphicAfterOperations(Vec<PreviewOperationId>),
}

#[derive(Debug)]
pub enum RawGraphClassRelationDefinition {
    Text(String),
    GraphRelation(PreviewGraphRelationId),
}

#[derive(Debug)]
pub enum RawGraphClassDefinition {
    Text(String),
    IntersectionGraphClasses(Vec<PreviewGraphClassId>),
    IntersectionGraphClassProperty(PreviewGraphClassId, PreviewGraphClassPropertyId),
    ParametricGraphClass(PreviewParametricGraphClassId),
    Parameter(PreviewParameterId),
}

#[derive(Debug)]
pub enum RawGraphClassVariant {
    GraphClass,
    GraphProperty,
}

#[derive(Debug)]
pub enum RawParametricParameterDefinition {
    GraphClassParameter(String),
    GraphParameter(String),
}
