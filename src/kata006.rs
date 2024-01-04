// A Narcissistic Number (or Armstrong Number) is a positive number which is the
// sum of its own digits, each raised to the power of the number of digits in a
// given base. In this Kata, we will restrict ourselves to decimal (base 10).
//
// For example, take 153 (3 digits), which is narcissistic:
//    1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
//
// and 1652 (4 digits), which isn't:
//    1^4 + 6^4 + 5^4 + 2^4 = 1 + 1296 + 625 + 16 = 1938
//
// The Challenge:
//
// Your code must return true or false (not 'true' and 'false') depending upon
// whether the given number is a Narcissistic number in base 10.
//
// This may be True and False in your language, e.g. PHP.
//
// Error checking for text strings or other invalid inputs is not required, only
// valid positive non-zero integers will be passed into the function.
#[allow(dead_code)]
fn narcissistic(num: u64) -> bool {
    let str_num = num.to_string();
    let total = str_num.chars()
        .map(|n| n.to_digit(10).unwrap() as u64)
        .map(|n| { n.pow(str_num.len() as u32) })
        .sum::<u64>();
    total == num
}

// ------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(input: u64, expected: bool) {
        let actual = narcissistic(input);
        assert_eq!(
            actual, expected,
            "\nIncorrect answer for n={}\nExpected: {expected}\nActual: {actual}",
            input
        )
    }

    #[test]
    fn basic_tests() {
        dotest(7, true);
        dotest(371, true);
        dotest(122, false);
        dotest(4887, false);
    }
}
