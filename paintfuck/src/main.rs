use std::iter;
use itertools::Itertools;

fn zero_vec(size: usize) -> Vec<char> {
    iter::repeat('0').take(size).collect_vec()
}

fn zero_grid(width: usize, height: usize) -> Vec<Vec<char>> {
    (0..height).map(|_| zero_vec(width)).collect_vec()
}

fn to_string(grid: &Vec<Vec<char>>) -> String {
    grid.iter().map(|r| r.iter().collect::<String>()).join("\r\n")
}

fn offset_to<'a, T: Iterator<Item=&'a char>>(iter: T, target: char, source: char) -> usize {
    let mut count = 0;
    for (i, c) in iter.enumerate() {
        // println!("offset_to: i: {}, c: {}, count: {}", i, c, count);
        match *c {
            t if t == target && count == 0 => return i + 1,
            t if t == target && count > 0 => count -= 1,
            s if s == source => count += 1,
            _ => ()
        }
    }
    return 1;
}

fn interpreter(code: &str, iterations: usize, width: usize, height: usize) -> String {
    // println!("Interepter!");
    let mut grid = zero_grid(width, height);
    let code = code.chars().collect_vec();

    let mut i = 0;
    let mut c: usize = 0;
    let mut x = 0;
    let mut y = 0;
    while c < code.len() && i < iterations {
        // println!("c={}, i={}, x={}, y={}, code={}", c, i, x, y, code[c]);
        match code[c] {
            'n' => y = if y == 0 { height - 1} else { y - 1},
            'e' => x = if x == width - 1 { 0 } else { x + 1},
            's' => y = if y == height - 1 { 0 } else { y + 1},
            'w' => x = if x == 0 { width - 1} else { x - 1},
            '*' => grid[y][x] = if grid[y][x] == '0' { '1' } else { '0' },
            _ => ()
        }
        match code[c] {
            '[' if grid[y][x] == '0' => {
                c += offset_to( code[(c + 1)..].iter(), ']', '[') + 1;
                i += 1
            }
            ']' if grid[y][x] != '0' => {
                c -= offset_to(code[..c].iter().rev(), '[', ']') - 1;
                i += 1
            }
            'n' | 'e' | 's' | 'w' | '*' | '['  => {
                c += 1;
                i += 1;
            }
            _ => c += 1,
        }
        // println!("{}", to_string(&grid));
    }

    to_string(&grid)
}

#[test]
fn simple_cases() {
    // println!("\n\n{}", interpreter("*[s[e]*]", 1000, 10, 10));
    println!("\n\n{}", interpreter("* [[s*en] sw[w]enn [[e]w*ssss*nnnnw[w]ess[e]*[w]enn] ssss[*nnnn*essss] nnnnw[w]e ss]", 100000, 75, 20));
    // assert_eq!(interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 0, 6, 9),
    //            "000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000",
    //            "Your interpreter should initialize all cells in the datagrid to 0");
    // assert_eq!(interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 7, 6, 9),
    //            "111100\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000",
    //            "Your interpreter should adhere to the number of iterations specified");
    // assert_eq!(interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 19, 6, 9),
    //            "111100\r\n000010\r\n000001\r\n000010\r\n000100\r\n000000\r\n000000\r\n000000\r\n000000",
    //            "Your interpreter should traverse the 2D datagrid correctly");
    // assert_eq!(interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 42, 6, 9),
    //            "111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000",
    //            "Your interpreter should traverse the 2D datagrid correctly for all of the \"n\", \"e\", \"s\" and \"w\" commands");
    // assert_eq!(interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 100, 6, 9),
    //            "111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000",
    //            "Your interpreter should terminate normally and return a representation of the final state of the 2D datagrid when all commands have been considered from left to right even if the number of iterations specified have not been fully performed");
}