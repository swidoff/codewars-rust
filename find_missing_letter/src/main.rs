use itertools::Itertools;

fn find_missing_letter(chars: &[char]) -> char {
    chars.iter()
        .map(|c| *c as u8)
        .tuple_windows()
        .find_map(|(b1, b2)| if b2 - b1 > 1 { Some((b1 + 1) as char) } else { None })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }
}