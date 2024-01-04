// This time no story, no theory. The examples below show you how to write
// function accum:
//
// Examples:
//   accum("abcd") -> "A-Bb-Ccc-Dddd"
//   accum("RqaEzty") -> "R-Qq-Aaa-Eeee-Zzzzz-Tttttt-Yyyyyyy"
//   accum("cwAt") -> "C-Ww-Aaa-Tttt"
//
// The parameter of accum is a string which includes only letters from a..z and A..Z.
#[allow(dead_code)]
fn accum(s: &str) -> String {
     s
        .to_lowercase()
        .chars()
        .map(String::from)
        .enumerate()
        .map(|(i, letter)| { letter.to_uppercase() + &letter.repeat(i) })
        .collect::<Vec<String>>()
        .join("-")
}

// ------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(
            accum("ZpglnRxqenU"),
            "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu"
        );
        assert_eq!(
            accum("NyffsGeyylB"),
            "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb"
        );
        assert_eq!(
            accum("MjtkuBovqrU"),
            "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu"
        );
        assert_eq!(
            accum("EvidjUnokmM"),
            "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm"
        );
        assert_eq!(
            accum("HbideVbxncC"),
            "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc"
        );
    }
}
