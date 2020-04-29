use itertools::Itertools;

// This one was a bitch.

fn next_bigger_number(n: i64) -> i64 {
    let mut s = n.to_string().chars().collect_vec();
    let index = s.iter()
        .rev()
        .tuple_windows()
        .enumerate()
        .find_map(|(i, (a, b))| if a > b { Some((s.len() - 1) - (i + 1)) } else { None });

    if let Some(i) = index {
        let mut subsequence: Vec<char> = Vec::from(&s[i..]);
        let (min_index, _) = subsequence
            .iter()
            .enumerate()
            .filter(|(_, v)| **v > s[i])
            .min_by_key(|(_, v)| **v).unwrap();

        let min_value = subsequence.remove(min_index);
        subsequence.sort();
        subsequence.insert(0, min_value);

        s.truncate(i);
        s.append(&mut subsequence);
        s.iter().collect::<String>().parse::<i64>().unwrap()
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(21, next_bigger_number(12));
        assert_eq!(531, next_bigger_number(513));
        assert_eq!(2071, next_bigger_number(2017));
        assert_eq!(441, next_bigger_number(414));
        assert_eq!(414, next_bigger_number(144));
        assert_eq!(-1, next_bigger_number(9));
        assert_eq!(-1, next_bigger_number(111));
        assert_eq!(-1, next_bigger_number(531));
        assert_eq!(1234567980, next_bigger_number(1234567908));
        assert_eq!(4276251410177800395, next_bigger_number(4276251410177800359));
        assert_eq!(4276251410177800539, next_bigger_number(4276251410177800395));
        assert_eq!(4276251410177803059, next_bigger_number(4276251410177800953));
    }
}
