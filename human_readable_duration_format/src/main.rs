const UNITS: [(&str, u64); 5] = [
    ("year", 365 * 24 * 60 * 60),
    ("day", 24 * 60 * 60),
    ("hour", 60 * 60),
    ("minute", 60),
    ("second", 1),
];

fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        return "now".to_string();
    }

    let mut parts: Vec<String> = Vec::new();
    let mut remainder = seconds;
    for (unit, unit_seconds) in UNITS.iter() {
        let scale = remainder / *unit_seconds;
        if scale > 0 {
            parts.push(format!("{} {}{}", scale, *unit, if scale > 1 { "s" } else { "" }));
            remainder = remainder % (scale * *unit_seconds);
        }
    }

    let mut res = String::new();
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            if i < parts.len() - 1 {
                res.push_str(", ");
            } else {
                res.push_str(" and ");
            }
        }
        res.push_str(part);
    }

    res
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(0), "now");
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
    }
}
