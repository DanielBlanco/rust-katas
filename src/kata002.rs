use itertools::Itertools;

// Write a function that will return the count of distinct case-insensitive
// alphabetic characters and numeric digits that occur more than once in the
// input string. The input string can be assumed to contain only alphabets 
// (both uppercase and lowercase) and numeric digits.
//
// Example
//   "abcde" -> 0 # no characters repeats more than once
//   "aabbcde" -> 2 # 'a' and 'b'
//   "aabBcde" -> 2 # 'a' occurs twice and 'b' twice (`b` and `B`)
//   "indivisibility" -> 1 # 'i' occurs six times
//   "Indivisibilities" -> 2 # 'i' occurs seven times and 's' occurs twice
//   "aA11" -> 2 # 'a' and '1'
//   "ABBA" -> 2 # 'A' and 'B' each occur twice
//
#[allow(dead_code)]
fn count_duplicates(text: &str) -> u32 {
    let mut found = std::collections::HashMap::new();
    let text = text.to_lowercase();
    for c in text.chars() {
        let v = if found.contains_key(&c) { 1 } else { 0 };
        found.insert(c, v);
    }
    found.values().sum()
}


// With itertools
#[allow(dead_code)]
fn count_duplicates2(text: &str) -> u32 {
    text.to_lowercase().chars().counts().values().filter(|&&i| i > 1).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }
    
    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates2("abcdea"), 1);
    }
    
    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates2("indivisibility"), 1);
    }
}
