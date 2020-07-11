pub struct StringCalculator {}

impl StringCalculator {
    pub fn add(&self, numbers: String) -> i32 {
        if numbers == "" {
            return 0;
        }

        let v: Vec<i32>;

        if self.has_custom_delimiter(&numbers) {
            let delim_and_nb: Vec<&str> = numbers.split('\n').collect();
            let custom_delimiter: String = delim_and_nb[0][2..].to_string();
            let numbers_to_add = delim_and_nb[1];

            v = numbers_to_add
                .split(&custom_delimiter)
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
        } else {
            let delimiters = vec![',', '\n'];
            v = numbers
                .split(|c| delimiters.contains(&c))
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
        }


        let negatives = v.iter().filter(|x| x.is_negative()).collect::<Vec<&i32>>();
        if negatives.iter().count() > 0 {
            panic!("negatives not allowed, received: {:?}", negatives);
        }

        v.iter().sum()
    }

    fn has_custom_delimiter(&self, numbers: &str) -> bool {
        &numbers.len() > &1 && &numbers[..2] == "//"
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

    #[test]
    fn changing_the_delimiter() {
        let sc = StringCalculator {};
        assert_eq!(sc.add("//;\n1;2".to_string()), 3);
    }

    #[test]
    #[should_panic(expected="negatives not allowed, received: [-1]")]
    fn throws_exception_on_negative_number() {
        let sc = StringCalculator {};
        sc.add("1,-1".to_string());
    }
    
}
