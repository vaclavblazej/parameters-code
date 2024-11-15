
use std::collections::HashMap;

use biblatex::Entry;

use crate::data::data::{Date, Relation, Showed, ShowedFact, Source};
use crate::general::enums::SourceKey;
use crate::input::raw::*;
use crate::data::preview::*;


impl Into<PreviewSourceKey> for RawSourceKey {
    fn into(self) -> PreviewSourceKey {
        match self {
            Self::Bibtex { key } => PreviewSourceKey::Bibtex { key },
            Self::Online { url } => PreviewSourceKey::Online { url },
            Self::Other { name, description: _ } => PreviewSourceKey::Other { name },
        }
    }
}

impl Into<PreviewKind> for RawKind {
    fn into(self) -> PreviewKind {
        match self {
            Self::Parameter => PreviewKind::Parameter,
            Self::GraphClass => PreviewKind::GraphClass,
        }
    }
}

impl Into<PreviewSet> for RawSet {
    fn into(self) -> PreviewSet {
        PreviewSet {
            id: self.id,
            name: self.name,
            kind: self.kind.into(),
            popularity: self.popularity,
            hidden: self.hidden,
        }
    }
}

impl RawShowed {
    pub fn preprocess(self, sourcekey: &SourceKey) -> Showed {
        Showed {
            id: self.id,
            text: self.text,
            fact: self.fact.preprocess(&sourcekey),
            page: self.page,
        }
    }
}

impl RawShowedFact {
    pub fn preprocess(self, sourcekey: &SourceKey) -> ShowedFact {
        match self {
            Self::Relation(x) => ShowedFact::Relation(x.into()),
            Self::Citation(x) => ShowedFact::Citation(x.preprocess(&sourcekey)),
            Self::Definition(x) => ShowedFact::Definition(x.into()),
        }
    }
}

impl Into<Relation> for RawRelation {
    fn into(self) -> Relation {
        Relation {
            id: self.clone().get_id(),
            preview: self.clone().into(),
            subset: self.subset.clone().into(),
            superset: self.superset.clone().into(),
            cpx: self.cpx.clone(),
            combined_from: None,
            essential: true,
        }
    }
}

impl Into<PreviewRelation> for RawRelation {
    fn into(self) -> PreviewRelation {
        PreviewRelation {
            id: self.get_id(),
            subset: self.subset.into(),
            superset: self.superset.into(),
            cpx: self.cpx,
        }
    }
}

impl RawSource {
    pub fn preprocess(self, sourcekey: &SourceKey) -> PreviewSource {
        PreviewSource {
            id: self.id,
            sourcekey: sourcekey.clone(),
            time: Date::empty(),
        }
    }
}
