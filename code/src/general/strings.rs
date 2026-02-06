//! Utility functions for string formatting.

pub fn nice_concat<V>(elements: Vec<V>) -> String
where
    V: std::fmt::Display,
{
    let mut res = String::new();
    for (i, element) in elements.iter().enumerate() {
        let join = if i == 0 {
            ""
        } else if elements.len() == 2 {
            " and "
        } else if i + 1 == elements.len() {
            ", and "
        } else {
            ", "
        };
        res += &format!("{}{}", join, element);
    }
    res
}
