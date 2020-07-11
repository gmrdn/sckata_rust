pub struct StringCalculator {}

impl StringCalculator {
    pub fn add(&self, numbers: String) -> i32 {
        if numbers == "" {
            return 0;
        }

        let v: Vec<i32> = numbers
            .split(|c| c == ',' || c == '\n')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        v.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adding_zero_numbers_returns_zero() {
        let sc = StringCalculator {};
        assert_eq!(sc.add("".to_string()), 0);
    }

    #[test]
    fn adding_one_number() {
        let sc = StringCalculator {};
        assert_eq!(sc.add("1".to_string()), 1);
    }

    #[test]
    fn adding_two_numbers() {
        let sc = StringCalculator {};
        assert_eq!(sc.add("1,2".to_string()), 3);
    }

    #[test]
    fn adding_any_amount_of_numbers() {
        let sc = StringCalculator {};
        assert_eq!(sc.add("2,2,2".to_string()), 6);
        assert_eq!(sc.add("1,1,1,1,1,1,1".to_string()), 7);
    }

    #[test]
    fn adding_numbers_with_newline_separator() {
        let sc = StringCalculator {};
        assert_eq!(sc.add("1\n2,3".to_string()), 6);
    }
}
