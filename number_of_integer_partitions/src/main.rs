use std::collections::HashMap;

fn partitions(n: isize) -> isize {
    let mut cache: HashMap<(usize, usize), isize> = HashMap::new();
    partitions_recurse(n as usize, 1, &mut cache)
}

fn partitions_recurse(n: usize, threshold: usize, cache: &mut HashMap<(usize, usize), isize>) -> isize {
    if let Some(value) = cache.get(&(n, threshold)) {
        *value
    } else {
        let res = 1 + (threshold..(n / 2) + 1)
            .fold(0, |sum, i|
                sum + partitions_recurse(n - i, i, cache)
            );
        cache.insert((n, threshold), res);
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_01() {
        assert_eq!(partitions(1), 1);
    }

    #[test]
    fn basic_test_05() {
        assert_eq!(partitions(5), 7);
    }

    #[test]
    fn basic_test_06() {
        assert_eq!(partitions(6), 11);
    }

    #[test]
    fn basic_test_10() {
        assert_eq!(partitions(10), 42);
    }

    #[test]
    fn basic_test_25() {
        assert_eq!(partitions(25), 1958);
    }

    #[test]
    fn basic_test_100() {
        assert_eq!(partitions(100), 190569292);
    }
}