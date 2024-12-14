use super::{build::Builder, raw::{RawData, RawSet, RawTopic}};


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

    pub fn topic(mut self, topic: &RawTopic) -> Self {
        self.set.topics.push(topic.clone());
        self
    }

    pub fn done(mut self) -> RawSet {
        self.builder.add_set(&self.set);
        self.set
    }
}

