fn to_int_vec(counts: &str) -> Vec<i32> {
    counts.split_whitespace().map(|n| n.parse::<i32>().unwrap_or(0)).collect()
}

fn inner_product(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
    v1.iter().zip(v2.iter()).fold(0, |sum, (n1, n2)| sum + n1*n2)
}

fn good_vs_evil(good: &str, evil: &str) -> String {
    let good_wt = vec![1, 2, 3, 3, 4, 10];
    let evil_wt = vec![1, 2, 2, 2, 3, 4, 10];
    let good_score = inner_product(good_wt, to_int_vec(good));
    let evil_score = inner_product(evil_wt, to_int_vec(evil));

    if good_score > evil_score {
        return "Battle Result: Good triumphs over Evil".to_string()
    } else if evil_score > good_score {
        "Battle Result: Evil eradicates all trace of Good".to_string()
    } else {
        "Battle Result: No victor on this battle field".to_string()
    }
}

#[test]
fn returns_expected() {
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
    assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
}