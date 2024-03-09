
use biblatex::Entry;

use crate::data::data::{Date, Relation, Showed, ShowedFact};
use crate::input::raw::*;
use crate::data::preview::*;


impl Into<PreviewSourceKey> for RawSourceKey {
    fn into(self) -> PreviewSourceKey {
        match self {
            Self::Bibtex { key } => PreviewSourceKey::Bibtex { key },
            Self::Online { url } => PreviewSourceKey::Online { url },
            Self::Unknown => PreviewSourceKey::Unknown,
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
        }
    }
}

impl Into<Showed> for RawShowed {
    fn into(self) -> Showed {
        Showed {
            id: self.id,
            text: self.text,
            fact: self.fact.into(),
            page: self.page,
        }
    }
}

impl Into<ShowedFact> for RawShowedFact {
    fn into(self) -> ShowedFact {
        match self {
            Self::Relation(x) => ShowedFact::Relation(x.into()),
            Self::Citation(x) => ShowedFact::Citation(x.preprocess()),
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
    pub fn preprocess(self) -> PreviewSource {
        PreviewSource {
            id: self.id,
            sourcekey: self.rawsourcekey.into(),
            time: Date::empty(),
        }
    }
}
