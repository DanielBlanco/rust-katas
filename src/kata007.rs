// ROT13 is a simple letter substitution cipher that replaces a letter with the
// letter 13 letters after it in the alphabet. ROT13 is an example of the Caesar
// cipher.
//
// Create a function that takes a string and returns the string ciphered with 
// Rot13. If there are numbers or special characters included in the string,
// they should be returned as they are. Only letters from the latin/english
// alphabet should be shifted, like in the original Rot13 "implementation".
#[allow(dead_code)]
fn rot13(message: &str) -> String {
    let alphabet = ('a'..='z').into_iter().collect::<Vec<char>>();
    message
        .chars()
        .map(|c| {
            if let Some(pos) = alphabet
                .iter()
                .position(|&x| x.to_string() == c.to_lowercase().to_string()) {
                    let x = alphabet.get((pos + 13) % 26).unwrap_or(&'a');
                    if c.is_uppercase() {
                        x.to_string().to_uppercase().to_owned() 
                    } else { x.to_string().to_owned() }
            } else { c.to_string() }
        })
        .collect()
}

#[allow(dead_code)]
fn rot13_best(message: &str) -> String {
    message.chars().map(|c| {
        match c {
            'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + 13) as char,
            'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - 13) as char,
            _ => c,
        }
    }).collect()
}

// ------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(s: &str, expected: &str) {
        assert_eq!(rot13_best(s), expected, "{ERR_MSG} with message = \"{s}\"")
    }

    #[test]
    fn sample_tests() {
        dotest("test", "grfg");
        dotest("Test", "Grfg");
    }
}
