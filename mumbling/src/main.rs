fn main() {
    println!("Hello, world!");
}

fn accum_mine(s: &str) -> String {
    let mut res = String::new();

    for (i, c) in s.chars().enumerate() {
        if i > 0 {
            res.push('-')
        }

        for c_upper in c.to_uppercase() {
            res.push(c_upper);
        }
        for _ in 1..(i + 1) {
            for c_lower in c.to_lowercase() {
                res.push(c_lower);
            }
        }
    }

    res
}

fn accum(s: &str) -> String {
    s.chars().enumerate()
        .map(|(i,c)| c.to_string().to_uppercase() + c.to_string().to_lowercase().repeat(i).as_str())
        .collect::<Vec<String>>()
        .join("-")

}

#[test]
fn basic_tests() {
    assert_eq!(accum("ZpglnRxqenU"), "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu");
    assert_eq!(accum("NyffsGeyylB"), "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb");
    assert_eq!(accum("MjtkuBovqrU"), "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu");
    assert_eq!(accum("EvidjUnokmM"), "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm");
    assert_eq!(accum("HbideVbxncC"), "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc");
}