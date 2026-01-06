use crate::data::id::*;

#[derive(Debug, PartialEq, Clone)]
pub enum RawOwn {
    Has,
    Is,
}

#[derive(Debug)]
pub enum RawParameterDefinition {
    Text(String),
    BoundsAll(PreviewParametricParameterId),
}

#[derive(Debug)]
pub enum RawOperationDefinition {
    GraphOperation(String),
    GraphClassOperation(String),
}

#[derive(Debug)]
pub enum RawGraphClassPropertyDefinition {
    Text(String),
    FromGraphClass(PreviewGraphClassId),
    FromParameter(PreviewParameterId),
}

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
    GraphRelation(RawGraphRelationDefinition),
}

#[derive(Debug)]
pub enum RawGraphClassDefinition {
    Text(Vec<String>),
    Intersection(Vec<PreviewGraphClassId>),
    ParametricGraphClass(PreviewParametricGraphClassId),
    Parameter(PreviewParameterId),
}

#[derive(Debug)]
pub enum RawParametricParameterDefinition {
    GraphClassParameter(String),
    GraphParameter(String),
}
