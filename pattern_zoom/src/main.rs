const OPEN: char = '□';
const CLOSED: char = '■';

fn zoom(n: i32) -> String {
    let n = n as usize;

    // Initialize result as a 2D vec of chars filled with the CLOSED char.
    let mut res: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        let mut row: Vec<char> = Vec::new();
        for _ in 0..n {
            row.push(CLOSED);
        }
        res.push(row);
    }

    // Fill every other square starting with the one just outside the center and working out.
    let midpoint = n / 2;
    (1..(midpoint + 1))
        .filter(|i| i % 2 == 1)
        .for_each(|i| {
            let min = midpoint - i;
            let max = midpoint + i;
            for j in min..(max + 1) {
                res[min][j] = OPEN;
                res[max][j] = OPEN;
                res[j][min] = OPEN;
                res[j][max] = OPEN;
            }
        });

    // Collapse the 2D vec into a String.
    res.iter()
        .map(|r| r.iter().collect::<String>())
        .collect::<Vec<String>>().join("\n")
}

#[test]
fn basic_test_1() {
    assert_eq!(zoom(1), "■");
}

#[test]
fn basic_test_2() {
    assert_eq!(zoom(3), "\
□□□
□■□
□□□"
    );
}

#[test]
fn basic_test_3() {
    assert_eq!(zoom(5), "\
■■■■■
■□□□■
■□■□■
■□□□■
■■■■■"
    );
}

#[test]
fn basic_test_4() {
    assert_eq!(zoom(7), "\
□□□□□□□
□■■■■■□
□■□□□■□
□■□■□■□
□■□□□■□
□■■■■■□
□□□□□□□"
    );
}

#[test]
fn basic_test_5() {
    assert_eq!(zoom(9), "\
■■■■■■■■■
■□□□□□□□■
■□■■■■■□■
■□■□□□■□■
■□■□■□■□■
■□■□□□■□■
■□■■■■■□■
■□□□□□□□■
■■■■■■■■■"
    );
}