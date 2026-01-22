//! General structure to infer relations.

use crate::data::id::*;

/// General relation between two structures along with some composable data.
#[derive(Debug)]
pub struct Relation<F, T, D> {
    pub from: F,
    pub to: T,
    pub data: D,
}

impl<F, T, D> Relation<F, T, D>
where
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
pub struct Hierarchy<VertexId, Data> {
    pub vertices: Vec<VertexId>,
    pub relations: Vec<Relation<VertexId, VertexId, Data>>,
}

impl<VertexId, Data> Hierarchy<VertexId, Data>
where
    VertexId: Clone,
    Data: ComposableHomogeneous<Data>,
{
    fn add_vertex(&mut self, id: VertexId) {
        self.vertices.push(id);
    }

    fn add_relation(&mut self, rel: Relation<VertexId, VertexId, Data>) {
        self.relations.push(rel)
    }

    fn get_relation(&self, a: VertexId, b: VertexId) {
        todo!()
    }
}
