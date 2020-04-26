fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
    where
        T: std::iter::IntoIterator,
        T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    sequence.into_iter().fold(Vec::new(), |mut v, i| {
        if v.last().map(|l| *l != i).unwrap_or(true) {
            v.push(i)
        }
        v
    })
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(unique_in_order("AAAABBBCCDAABBB".chars()), vec!['A', 'B', 'C', 'D', 'A', 'B']);
        assert_eq!(unique_in_order("ABBCcAD".chars()), vec!['A', 'B', 'C', 'c', 'A', 'D']);
        assert_eq!(unique_in_order(vec![1, 2, 2, 3, 3]), vec![1, 2, 3]);
        assert_eq!(unique_in_order(vec![1]), vec![1]);
        assert_eq!(unique_in_order::<Vec<i32>>(vec![]), vec![]);
    }
}