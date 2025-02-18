use super::{build::Builder, raw::{BuiltRawSet, RawData, RawSet, RawTag}};
use crate::general::enums::Page::NotApplicable;

pub struct SetBuilder<'a> {
    set: BuiltRawSet,
    builder: &'a mut Builder,
    later_operations: Vec<Box<dyn Fn(&mut Builder, &RawSet)>>,
}

impl<'a> SetBuilder<'a> {

    pub fn new(set: BuiltRawSet,
               builder: &'a mut Builder,
               ) -> Self {
        Self { builder, set, later_operations: vec![] }
    }

    pub fn aka(mut self, alternative_name: &str) -> Self {
        self.set.aka.push(alternative_name.into());
        self
    }

    pub fn abbr(mut self, abbreviation: &str) -> Self {
        assert!(self.set.abbr == None);
        self.set.abbr = Some(abbreviation.into());
        self
    }

    pub fn tag(mut self, tag: &RawTag) -> Self {
        self.set.tags.push(tag.clone());
        self
    }

    pub fn add_callback(mut self, callback: Box<dyn Fn(&mut Builder, &RawSet)>) -> Self {
        self.later_operations.push(callback);
        self
    }

    pub fn defined(mut self, id: &str, text: &str) -> Self {
        let id_local: String = id.into();
        let text_local: String = text.into();
        let res = self.add_callback(Box::new(move|builder: &mut Builder, raw_set: &RawSet|{
            builder.assumed_source().defined(id_local.as_str(), NotApplicable, raw_set, text_local.as_str());
        }));
        res
    }

    pub fn hide(mut self) -> Self {
        self.set.relevance = 0; // todo - unsure about whether to add hidden sets as an explicit property
        self
    }

    pub fn done(mut self) -> RawSet {
        let res: RawSet = self.set.into();
        self.builder.add_set(&res);
        for operation in &self.later_operations {
            operation(self.builder, &res);
        }
        res
    }

}

