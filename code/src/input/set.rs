use super::{build::Builder, raw::{RawData, RawSet, RawTag}};
use crate::general::enums::Page::NotApplicable;

pub struct SetBuilder<'a> {
    set: RawSet,
    builder: &'a mut Builder,
}

impl<'a> SetBuilder<'a> {

    pub fn new(set: RawSet, builder: &'a mut Builder) -> Self {
        Self { builder, set }
    }

    pub fn aka(mut self, alternative_name: &str) -> Self {
        self.set.aka.push(alternative_name.into());
        self
    }

    pub fn tag(mut self, tag: &RawTag) -> Self {
        self.set.tags.push(tag.clone());
        self
    }

    pub fn defined(mut self, id: &str, text: &str) -> Self {
        self.builder.assumed_source().defined(id, NotApplicable, &self.set, text);
        self
    }

    pub fn hide(mut self) -> Self {
        self.set.hidden = true;
        self
    }

    pub fn done(mut self) -> RawSet {
        self.builder.add_set(&self.set);
        self.set
    }

}

