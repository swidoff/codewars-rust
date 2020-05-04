use std::collections::HashMap;
use itertools::Itertools;
use std::cmp::Ordering;

struct Counter {
    letter: char,
    count: [usize; 2],
}

impl Counter {
    fn new(letter: char) -> Counter {
        Counter { letter, count: [0, 0] }
    }

    fn max_count(&self) -> usize {
        self.count[0].max(self.count[1])
    }

    fn max_index(&self) -> char {
        if self.count[0] > self.count[1] { '1' }
        else if self.count[1] > self.count[0] { '2' }
        else { '=' }
    }
}

fn count_lowercase(s: &str, i: usize, counters: &mut HashMap<char, Counter>) {
    s.chars()
        .filter(|c| c.is_lowercase() && c.is_alphabetic())
        .for_each(|c| {
            let counter = counters.entry(c).or_insert_with(|| Counter::new(c));
            counter.count[i] += 1
        })
}

fn mix(s1: &str, s2: &str) -> String {
    let mut counters: HashMap<char, Counter> = HashMap::new();
    count_lowercase(s1, 0, &mut counters);
    count_lowercase(s2, 1, &mut counters);

    counters.values()
        .filter(|c| c.max_count() > 1)
        .map(|c| format!("{}:{}", c.max_index(), c.letter.to_string().repeat(c.max_count())))
        .sorted_by(|c1, c2| {
            match Ord::cmp(&c1.len(), &c2.len()) {
                Ordering::Equal => Ord::cmp(c1, c2),
                o => o.reverse()
            }
        })
        .collect_vec()
        .join("/")
}

fn testing(s1: &str, s2: &str, exp: &str) -> () {
    assert_eq!(&mix(s1, s2), exp)
}

#[test]
fn basics_mix() {
    testing("Are they here", "yes, they are here",
            "2:eeeee/2:yy/=:hh/=:rr");
    testing("looping is fun but dangerous", "less dangerous than coding",
            "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg");
    testing(" In many languages", " there's a pair of functions",
            "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt");
    testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
    testing("codewars", "codewars", "");
    testing("A generation must confront the looming ", "codewarrs",
            "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr");
}