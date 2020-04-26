fn part_list(arr: Vec<&str>) -> String {
    // 1. Iterate over the indexes.
    // 2. For each index, partition the list into a tuple of two lists
    // 3. For format each tuple as a string
    // 4. Join the strings into a single string
    (1..arr.len())
        .map(|i| arr.split_at(i))
        .map(|(l, r)| format!("({}, {})", l.join(" "), r.join(" ")))
        .fold(String::new(), |mut s, tup| {
            s.push_str(&tup);
            s
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(arr: Vec<&str>, exp: &str) -> () {
        println!("arr: {:?}", arr);
        let ans = part_list(arr);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basis_tests() {
        dotest(vec!["I", "wish", "I", "hadn't", "come"],
               "(I, wish I hadn't come)(I wish, I hadn't come)(I wish I, hadn't come)(I wish I hadn't, come)");
        dotest(vec!["cdIw", "tzIy", "xDu", "rThG"],
               "(cdIw, tzIy xDu rThG)(cdIw tzIy, xDu rThG)(cdIw tzIy xDu, rThG)");
        dotest(vec![], "")
    }
}