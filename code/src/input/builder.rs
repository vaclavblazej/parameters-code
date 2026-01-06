use crate::data::data::NameCore;
use crate::input::build::CollectionBuilder;
use std::marker::PhantomData;

type BuilderCallback<R> = dyn Fn(&mut CollectionBuilder, &R);

pub struct Builder<T, R> {
    later_operations: Vec<Box<BuilderCallback<R>>>,
    built_struct: R,
    _marker: PhantomData<T>,
    // sub_build: Buildable<T,R>,
}

// trait Buildable<T,R> {
// fn done(self, builder: &mut Builder<R>) -> R;
// }

impl<T, R> Builder<T, R> {
    pub fn new(built_struct: R) -> Self {
        Self {
            later_operations: vec![],
            built_struct,
            _marker: PhantomData::new(),
        }
    }

    pub fn hide(mut self) -> Self {
        self.core.relevance = 0; // todo - unsure about whether to add hidden sets as an explicit property
        self
    }

    pub fn add_callback(mut self, callback: Box<BuilderCallback<R>>) -> Self {
        self.later_operations.push(callback);
        self
    }

    pub fn done(mut self, builder: &mut CollectionBuilder) -> R {
        // let BuiltRawSet {
        // id,
        // name,
        // typ,
        // composed,
        // relevance,
        // aka,
        // abbr,
        // tags,
        // displayed_definition,
        // } = self.set;
        let res = self.built_struct;
        for operation in &self.later_operations {
            operation(builder, &res);
        }
        let ret = res.id.preview();
        for tag in tags {
            builder.tag_set(tag, ret.clone());
        }
        builder.add_set(res);
        ret
    }
}

impl<T, R> Named for Builder<T, R> {
    fn aka(mut self, alternative_name: &str) -> Self {
        self.core.aka.push(alternative_name.into());
        self
    }

    fn abbr(mut self, abbreviation: &str) -> Self {
        assert!(self.core.abbr.is_none());
        self.core.abbr = Some(abbreviation.into());
        self
    }
}

// impl<T,R> Tagged for Builder<T,R> {
// pub fn tag(mut self, tag: &PreviewTagId) -> Self {
// self.set.tags.push(tag.clone());
// self
// }
// }

// pub fn displayed_definition(mut self, id: &str, text: &str) -> Self {
// self.set.displayed_definition.push(text.into());
// self
// }
