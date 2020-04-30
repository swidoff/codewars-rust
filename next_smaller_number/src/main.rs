use itertools::Itertools;

fn next_smaller_number(n: u64) -> Option<u64> {
    let mut s = n.to_string().chars().collect_vec();
    let index = s.iter()
        .rev()
        .tuple_windows()
        .enumerate()
        .find_map(|(i, (a, b))| if a < b { Some((s.len() - 1) - (i + 1)) } else { None });

    index.and_then(|i| {
        let mut subsequence: Vec<char> = Vec::from(&s[i..]);
        let (max_index, _) = subsequence
            .iter()
            .enumerate()
            .filter(|(_, v)| **v < s[i])
            .max_by_key(|(_, v)| **v).unwrap();

        let max_value = subsequence.remove(max_index);
        subsequence.sort();
        subsequence.reverse();
        subsequence.insert(0, max_value);

        s.truncate(i);
        s.append(&mut subsequence);
        if s[0] == '0' { None } else { s.iter().collect::<String>().parse::<u64>().ok() }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(Some(12), next_smaller_number(21));
        assert_eq!(Some(790), next_smaller_number(907));
        assert_eq!(Some(513), next_smaller_number(531));
        assert_eq!(None, next_smaller_number(1027));
        assert_eq!(Some(414), next_smaller_number(441));
        assert_eq!(Some(1234567890), next_smaller_number(1234567908));
        assert_eq!(Some(276251410177598300), next_smaller_number(276251410177800359));
    }
}
