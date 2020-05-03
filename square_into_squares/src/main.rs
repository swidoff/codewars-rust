fn decompose(n: i64) -> Option<Vec<i64>> {
    do_decompose(n * n, n - 1, &Vec::new()).map(|v| {
        let mut res = v.clone();
        res.reverse();
        res
    })
}

fn do_decompose(target: i64, start: i64, res: &Vec<i64>) -> Option<Vec<i64>> {
    println!("do_decompose: target: {}, start: {}, res: {:?}", target, start, res);
    if target == 0 {
        Some(res.clone())
    } else if target < 0 || start < 1 {
        None
    } else {
        (1..(start + 1)).rev().find_map(|n| {
            let new_target = target - n * n;
            let new_start = ((new_target as f64).sqrt().floor() as i64).min(n - 1);
            let mut new_res = res.clone();
            new_res.push(n);
            do_decompose(new_target, new_start, &new_res)
        })
    }
}

fn testing(n: i64, exp: Option<Vec<i64>>) -> () {
    assert_eq!(decompose(n), exp)
}

#[test]
fn tests_decompose() {
    testing(50, Some(vec![1, 3, 5, 8, 49]));
    testing(44, Some(vec![2, 3, 5, 7, 43]));
    testing(625, Some(vec![2, 5, 8, 34, 624]));
    testing(5, Some(vec![3, 4]));
}