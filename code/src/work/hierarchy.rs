//! General structure to infer relations.

use crate::data::id::*;

pub trait Entity {
    type PreviewId;
}

/// General relation between two structures along with some composable data.
#[derive(Debug)]
pub struct Relation<F: Entity, T: Entity, D> {
    pub from: F::PreviewId,
    pub to: T::PreviewId,
    pub data: D,
}

impl<F: Entity, T: Entity, D> Relation<F, T, D>
where
    F::PreviewId: Clone,
    T::PreviewId: Clone,
    D: ComposableHomogeneous<D>,
{
    pub fn new(from: &F::PreviewId, to: &T::PreviewId, data: D) -> Relation<F, T, D> {
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

// #[derive(Debug)]
// pub struct Hierarchy {
//     pub vertices: Vec<Entity>,
//     pub relations: Vec<Relation<Entity::PreviewId, Entity::PreviewId, Data>>,
// }
//
// impl<Entity, Data> Hierarchy<Entity, Data>
// where
//     Entity: Clone,
//     Data: ComposableHomogeneous<Data>,
// {
//     fn add_vertex(&mut self, id: Entity) {
//         self.vertices.push(id);
//     }
//
//     fn add_relation(&mut self, rel: Relation<Entity, Entity, Data>) {
//         self.relations.push(rel)
//     }
//
//     fn get_relation(&self, a: Entity, b: Entity) {
//         todo!()
//     }
// }
