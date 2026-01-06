pub fn nice_concat<T>(sets: T) -> String {
    let mut res = String::new();
    for (i, set_id) in sets.iter().enumerate() {
        let join = if i + 1 == sets.len() {
            ", and"
        } else if i == 0 || sets.len() == 2 {
            " "
        } else {
            ", "
        };
        res += &format!("{} [[{}]]", join, set_id);
    }
    res
}
