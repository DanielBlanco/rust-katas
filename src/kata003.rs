/* Your task is to sort a given string. Each word in the string will contain a
 * single number. This number is the position the word should have in the result.
 *
 * Note: Numbers can be from 1 to 9. So 1 will be the first word (not 0).
 *
 * If the input string is empty, return an empty string. The words in the input
 * String will only contain valid consecutive numbers.
 */
#[allow(dead_code)]
fn order(sentence: &str) -> String {
    let mut words: Vec<(u32, &str)> = sentence.split_whitespace().map(|word| {
        let v: u32 = (1..=9)
            .find(|n| { word.contains(&n.to_string()) })
            .unwrap_or(0);
        (v, word)
    }).collect();

    // mutates words
    words.sort_by(|a, b| { (a.0).cmp(&b.0) });

    let o : Vec<String> = words.into_iter().map(|x| { x.1.to_string() }).collect();

    o.join(" ")
}

// Better
#[allow(dead_code)]
fn order2(sentence: &str) -> String {
    let mut ws: Vec<_> = sentence.split_whitespace().map(String::from).collect();
    ws.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
    ws.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(order2("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
