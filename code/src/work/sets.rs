use serde::{Deserialize, Serialize};

/// Minimal and maximal refer to inclusion-wise extremes. An isolated element
/// would be included in all three sets.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Sets<T> {
    pub minimal: Vec<T>,
    pub maximal: Vec<T>,
    pub all: Vec<T>,
}

impl Sets<T> {
    fn new(minimal: Vec<T>, maximal: Vec<T>, all: Vec<T>) -> Self {
        Self {
            minimal,
            maximal,
            all,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct RelatedSets<T> {
    pub equivsets: Vec<T>,
    pub supersets: Sets<T>,
    pub subsets: Sets<T>,
    pub super_exclusions: Sets<T>,
    pub sub_exclusions: Sets<T>,
    pub unknown: Sets<T>,
}

impl<T> RelatedSets<T> {
    pub fn new(
        equivsets: Vec<T>,
        supersets: Sets<T>,
        subsets: Sets<T>,
        super_exclusions: Sets<T>,
        sub_exclusions: Sets<T>,
        unknown: Sets<T>,
    ) -> Self {
        Self {
            equivsets,
            supersets,
            subsets,
            super_exclusions,
            sub_exclusions,
            unknown,
        }
    }
}

fn prepare_extremes<T>(preview_set: Vec<PreviewSet>, data: &SimpleIndex) -> Sets<T> {
    let mut minimal = Vec::new();
    let mut maximal = Vec::new();
    let mut all = Vec::new();
    for current_set in &preview_set {
        let mut is_maximal = true;
        let mut is_minimal = true;
        for other_set in &preview_set {
            if current_set != other_set {
                let ab = data.first_subset_of_second(current_set, other_set);
                let ba = data.first_subset_of_second(other_set, current_set);
                if ab && !ba {
                    is_minimal = false;
                }
                if ba && !ab {
                    is_maximal = false;
                }
            }
        }
        if is_maximal {
            maximal.push(current_set.clone());
        }
        if is_minimal {
            minimal.push(current_set.clone());
        }
        all.push(current_set.clone());
    }
    Sets::new(minimal, maximal, all)
}
