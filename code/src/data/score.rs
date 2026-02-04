use crate::data::id::HasId;

pub trait Score {
    fn score(&self) -> u32;
    fn set_score(&mut self, new_score: u32);

    fn hide(&mut self) {
        self.set_score(0);
    }
}

pub fn has_better_score_than<A, B>(a: &A, b: &B) -> bool
where
    A: Score + HasId,
    B: Score + HasId,
{
    (a.score() == b.score() && a.id() < b.id()) || a.score() > b.score()
}
