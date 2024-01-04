use std::collections::HashMap;

// The goal of this exercise is to convert a string to a new string where each
// character in the new string is "(" if that character appears only once in the
// original string, or ")" if that character appears more than once in the
// original string. Ignore capitalization when determining if a character is a
// duplicate.
//
// Examples
//
// "din"      =>  "((("
// "recede"   =>  "()()()"
// "Success"  =>  ")())())"
// "(( @"     =>  "))(("
//
#[allow(dead_code)]
fn duplicate_encode(word: &str) -> String {
    let word = word.to_lowercase();
    let mut hmap: HashMap<char, u32> = HashMap::new();
    let mut result = String::new();

    for letter in word.chars() {
        let count: u32 = **hmap.get(&letter).get_or_insert(&0);
        hmap.insert(letter, count + 1);
    }

    for letter in word.chars() {
        let v = *hmap.get(&letter).get_or_insert(&1);
        if v > &1 {
            result.push(')');
        } else {
            result.push('(');
        }
    }

    result
}

// Better solution.
#[allow(dead_code)]
fn duplicate_encode2(word: &str) -> String {
    let word = word.to_lowercase();
    word.chars()
        .map(|c| {
            if word.matches(c).count() == 1 {
                '('
            } else {
                ')'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_encode_tests() {
        assert_eq!(duplicate_encode2("din"), "(((");
        assert_eq!(duplicate_encode2("recede"), "()()()");
        assert_eq!(
            duplicate_encode2("Success"),
            ")())())",
            "should ignore case"
        );
        assert_eq!(duplicate_encode("(( @"), "))((");
    }
}
