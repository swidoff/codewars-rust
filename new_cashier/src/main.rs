use std::iter;
use itertools::Itertools;

fn get_order(input: String) -> String {
    [
        ("burger", "Burger"), ("fries", "Fries"),
        ("chicken", "Chicken"), ("pizza", "Pizza"),
        ("sandwich", "Sandwich"), ("onionrings", "Onionrings"),
        ("milkshake", "Milkshake"), ("coke", "Coke"),
    ].iter().flat_map(|(lower_word, upper_word)| {
        iter::repeat(*upper_word).take(input.matches(lower_word).count())
    }).collect_vec().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(get_order("milkshakepizzachickenfriescokeburgerpizzasandwichmilkshakepizza".to_string()),
                   "Burger Fries Chicken Pizza Pizza Pizza Sandwich Milkshake Milkshake Coke".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(get_order("pizzachickenfriesburgercokemilkshakefriessandwich".to_string()),
                   "Burger Fries Fries Chicken Pizza Sandwich Milkshake Coke".to_string());
    }
}