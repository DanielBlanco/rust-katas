// Build Tower
// Build a pyramid-shaped tower, as an array/list of strings, given a positive integer number of floors. A tower block is represented with "*" character.

// For example, a tower with 3 floors looks like this:
//
// [
//   "  *  ",
//   " *** ",
//   "*****"
// ]
//
// And a tower with 6 floors looks like this:
//
// [
//   "     *     ",
//   "    ***    ",
//   "   *****   ",
//   "  *******  ",
//   " ********* ",
//   "***********"
// ]
#[allow(dead_code)]
fn tower_builder(n_floors: usize) -> Vec<String> {
    (1..=n_floors)
        .map(|i| {
            format!(
                "{}{}{}",
                " ".repeat(n_floors - i),
                "*".repeat((2 * i) - 1),
                " ".repeat(n_floors - i),
            )
        })
        .collect::<Vec<String>>()
}

// ------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_tests() {
        assert_eq!(tower_builder(1), vec!["*"]);
        assert_eq!(tower_builder(2), vec![" * ", "***"]);
        assert_eq!(tower_builder(3), vec!["  *  ", " *** ", "*****"]);
    }
}
