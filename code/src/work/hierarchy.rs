//! General structure to infer relations.

use std::marker::PhantomData;

use crate::data::id::*;


/// General relation between two structures along with some composable data.
#[derive(Debug)]
pub struct Relation<F, T, D> {
    pub from: F,
    pub to: T,
    pub data: D,
}

impl<F, T, D> Relation<F, T, D> where
    F: Clone,
    T: Clone,
    D: ComposableHomogeneous<D>,
{
    pub fn new(from: &F, to: &T, data: D) -> Relation<F, T, D> {
        Relation {
            from: from.clone(),
            to: to.clone(),
            data,
        }
    }
}

pub trait ComposableHomogeneous<T> {
    fn combine_serial(first: T, second: T) -> Option<T>;
    fn combine_parallel(first: T, second: T) -> Option<T>;
}

pub trait ComposableHeterogeneousFirst<A, B> {
    fn combine_serial(first: A, second: B) -> A;
}

pub trait ComposableHeterogeneousSecond<A, B> {
    fn combine_serial(first: A, second: B) -> B;
}

#[derive(Debug)]
pub struct Hierarchy<NodeId, Rel, Data> {
    pub nodes: Vec<NodeId>,
    pub relations: Vec<Rel>,
    _marker: PhantomData<Data>,
}

impl<NodeId, Rel, Data> Hierarchy<NodeId, Rel, Data> where
    NodeId: Clone,
    Rel: Relation<NodeId, NodeId, Data>,
    Data: ComposableHomogeneous<Data>,
{
}
