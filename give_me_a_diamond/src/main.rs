use std::iter;

fn print(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        None
    } else {
        Some(
            (0..n).flat_map(|i| {
                let spaces = (n / 2 - i).abs();
                let stars = (n - spaces * 2).abs();
                (0..spaces).map(|_| ' ')
                    .chain((0..stars).map(|_| '*'))
                    .chain(iter::once('\n'))
            }).collect::<String>()
        )
    }
}

#[test]
fn basic_test() {
    assert_eq!(print(3), Some(" *\n***\n *\n".to_string()));
    assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()));
    assert_eq!(print(-3), None);
    assert_eq!(print(2), None);
    assert_eq!(print(0), None);
    assert_eq!(print(1), Some("*\n".to_string()));
}