// fn main() {
//     println!("Hello, world!");
// }

fn order(sentence: &str) -> String {
    let mut words = sentence.split(' ').collect::<Vec<&str>>();
    words.sort_by_cached_key(|word| extract_digit(word));
    words.join(" ")
}

fn extract_digit(word: &str) -> u32 {
    word.chars()
        .find(char::is_ascii_digit)
        .and_then(|c| c.to_digit(10))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}