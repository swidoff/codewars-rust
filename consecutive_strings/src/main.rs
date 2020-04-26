fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let n = strarr.len();
    if k <= 0 || n == 0 || k > n {
        "".to_string()
    } else {
        let mut sum = 0;
        let mut max = 0;
        let mut index = 0;
        for (i, s) in strarr.iter().enumerate() {
            sum += s.len();
            if i >= k {
                sum -= strarr[i - k].len()
            }
            if sum > max {
                max = sum;
                index = i;
            }
        }

        strarr[(index - (k - 1))..(index + 1)].join("")
    }
}

fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&longest_consec(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    testing(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2, "abigailtheta");
    testing(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1,
            "oocccffuucccjjjkkkjyyyeehh");
    testing(vec![], 3, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 3, "ixoyx3452zzzzzzzzzzzz");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}