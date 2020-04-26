const TABLE: [[i32; 4]; 10] = [
    [0, 0, 0, 0],
    [1, 1, 1, 1],
    [2, 4, 8, 6],
    [3, 9, 7, 1],
    [4, 6, 4, 6],
    [5, 5, 5, 5],
    [6, 6, 6, 6],
    [7, 9, 3, 1],
    [8, 4, 2, 6],
    [9, 1, 9, 1],
];

fn last_digits_for(num: &str, digits: usize) -> i32 {
    let res = num[(num.len() - digits)..num.len()].parse::<i32>().unwrap_or(0);
    if res == 0 && digits > 1 { 100 } else { res }
}

fn last_digit(str1: &str, str2: &str) -> i32 {
    if str2 == "0" {
        if str1 == "0" { 0 } else { 1 }
    } else {
        let x = last_digits_for(str1, 1) as usize;
        let y = last_digits_for(str2, 2.min(str2.len()));
        println!("x:{} y:{}", x, y);
        TABLE[x][((y - 1) % 4) as usize]
    }
}

#[test]
fn returns_expected() {
    assert_eq!(last_digit("0", "0"), 0);
    assert_eq!(last_digit("4", "0"), 1);
    assert_eq!(last_digit("4", "1"), 4);
    assert_eq!(last_digit("4", "2"), 6);
    assert_eq!(last_digit("9", "7"), 9);
    assert_eq!(last_digit("10", "10000000000"), 0);
    assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376", "2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
    assert_eq!(last_digit("3715290469715693021198967285016729344580685479654510946723", "68819615221552997273737174557165657483427362207517952651"), 7);
}