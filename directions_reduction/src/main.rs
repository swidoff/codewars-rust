#[derive(PartialEq, Debug, Copy, Clone)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {
    fn cancels(&self, other: &Direction) -> bool {
        use Direction::*;
        match (*self, *other) {
            (n, s) | (s, n) if n == NORTH && s == SOUTH  => true,
            (e, w) | (w, e) if e == EAST && w == WEST  => true,
            _ => false
        }
    }
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut res: Vec<Direction> = Vec::new();

    for d in arr.into_iter() {
        if res.last().map(|l| d.cancels(l)).unwrap_or(false) {
            res.pop();
        } else {
            res.push(*d);
        }
    }
    res
}

#[test]
fn returns_expected() {
    use Direction::*;
    let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH, WEST];
    assert_eq!(dir_reduc(&a), [WEST]);
    let a = [NORTH, WEST, SOUTH, EAST];
    assert_eq!(dir_reduc(&a), [NORTH, WEST, SOUTH, EAST]);
}