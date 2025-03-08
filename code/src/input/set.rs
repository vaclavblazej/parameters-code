use super::{
    build::Builder,
    raw::{BuiltRawSet, RawData, RawSet, RawTag},
};
use crate::{
    data::id::{PreviewSetId, PreviewTagId},
    general::enums::Page::NotApplicable,
};

type SetBuilderCallback = dyn Fn(&mut Builder, &RawSet);

pub struct SetBuilder {
    set: BuiltRawSet,
    later_operations: Vec<Box<SetBuilderCallback>>,
}

impl SetBuilder {
    pub fn new(set: BuiltRawSet) -> Self {
        Self {
            set,
            later_operations: vec![],
        }
    }

    pub fn aka(mut self, alternative_name: &str) -> Self {
        self.set.aka.push(alternative_name.into());
        self
    }

    pub fn abbr(mut self, abbreviation: &str) -> Self {
        assert!(self.set.abbr.is_none());
        self.set.abbr = Some(abbreviation.into());
        self
    }

    pub fn tag(mut self, tag: &PreviewTagId) -> Self {
        self.set.tags.push(tag.clone());
        self
    }

    pub fn add_callback(mut self, callback: Box<SetBuilderCallback>) -> Self {
        self.later_operations.push(callback);
        self
    }

    pub fn defined(mut self, id: &str, text: &str) -> Self {
        let id_local: String = id.into();
        let text_local: String = text.into();
        let res = self.add_callback(Box::new(move |builder: &mut Builder, raw_set: &RawSet| {
            builder.assumed_source().ref_defined(
                id_local.as_str(),
                NotApplicable,
                &raw_set.id.preview(),
                text_local.as_str(),
            );
        }));
        res
    }

    pub fn hide(mut self) -> Self {
        self.set.relevance = 0; // todo - unsure about whether to add hidden sets as an explicit property
        self
    }

    pub fn done(self, builder: &mut Builder) -> PreviewSetId {
        let BuiltRawSet {
            id,
            name,
            typ,
            composed,
            relevance,
            aka,
            abbr,
            tags,
        } = self.set;
        let res = RawSet { id, name, typ, composed, relevance, aka, abbr, };
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
