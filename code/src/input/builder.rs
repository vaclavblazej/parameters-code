use crate::data::data::{NameCore, Named, Relevant, Tagged};
use crate::data::enums::Page;
use crate::data::id::{HasId, HasPreviewId, PreviewGraphClassId, PreviewTagId};
use crate::input::build::CollectionBuilder;
use crate::input::raw::{RawDataAddable, RawGraphClass};
use std::marker::PhantomData;

type BuilderCallback<R> = dyn Fn(&mut CollectionBuilder, &R);

pub struct Builder<Built> {
    later_operations: Vec<Box<BuilderCallback<Built>>>,
    built_struct: Box<Built>,
}

impl<Built> Builder<Built>
where
    Built: HasPreviewId + RawDataAddable + Named + HasId,
{
    pub fn new(built_struct: Built) -> Self {
        Self {
            later_operations: vec![],
            built_struct: Box::new(built_struct),
        }
    }

    pub fn add_callback(mut self, callback: Box<BuilderCallback<Built>>) -> Self {
        self.later_operations.push(callback);
        self
    }

    pub fn done(mut self, builder: &mut CollectionBuilder) -> Built::PreviewId {
        let res = self.built_struct;
        for operation in &self.later_operations {
            operation(builder, &res);
        }
        let ret = res.preview();
        builder.add_set(*res);
        ret
    }
}

// impl<R> Builder<R>
// where
//     R: Relevant,
// {
//     pub fn hide(mut self) -> Self {
//         self.hide();
//         self
//     }
// }

impl<R> Builder<R>
where
    R: Named,
{
    pub fn aka(mut self, alternative_name: &str) -> Self {
        self.built_struct.aka(alternative_name.into());
        self
    }
    pub fn abbr(mut self, abbreviation: &str) -> Self {
        self.built_struct.abbr(abbreviation.into());
        self
    }
}

impl<R> Builder<R>
where
    R: Tagged<PreviewTagId>,
{
    pub fn tag(mut self, tag: &PreviewTagId) -> Self {
        self.built_struct.add_tag(tag.clone());
        self
    }
}
