fn find_first(s: &str) -> i32 {
    for c in s.chars() {
        if c >= '0' && c <= '9' {
            return c.to_digit(10).unwrap() as i32;
        }
    }
    return -1;
}

fn find_last(s: &str) -> i32 {
    for c in s.chars().rev() {
        if c >= '0' && c <= '9' {
            return c.to_digit(10).unwrap() as i32;
        }
    }
    return -1;
}

fn calculate_sum_calibrations(lines: Vec<&str>) -> i32 {
    let mut total = 0;
    for line in lines {
        let first = find_first(line);
        let second = find_last(line);
        total += first * 10 + second;
    }
    return total;
}

fn main() {
    println!("todo later");
}

mod tests {
    use super::*;

    #[test]
    fn test1() {
        let tc = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        assert_eq!(calculate_sum_calibrations(tc), 142);
    }
}
