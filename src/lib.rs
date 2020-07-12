#[derive(Default)]
pub struct StringCalculator {
    nb_add_calls: i32,
}

impl StringCalculator {
    pub fn add(&mut self, numbers: String) -> i32 {
        self.increment_nb_add_calls();

        if numbers == "".to_string() {
            return 0;
        }

        let v = if self.has_custom_delimiter(&numbers) {
            let delim_and_nb: Vec<&str> = numbers.split('\n').collect();
            let custom_delimiter: String = delim_and_nb[0][2..].to_string();
            let numbers_to_add = delim_and_nb[1];

            numbers_to_add
                .split(&custom_delimiter)
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        } else {
            let delimiters = vec![',', '\n'];
            numbers
                .split(|c| delimiters.contains(&c))
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        };


        self.check_negatives(&v);


        v.iter().filter(|x| **x <= 1000).sum()
    }

    pub fn get_called_count(&self) -> i32 {
        self.nb_add_calls
    }

    fn increment_nb_add_calls(&mut self) {
        self.nb_add_calls += 1;
    }
    fn has_custom_delimiter(&self, numbers: &str) -> bool {
        numbers.len() > 1 && &numbers[..2] == "//"
    }

    fn check_negatives(&self, v: &Vec<i32>) {
        let negatives = v.iter().filter(|x| x.is_negative()).collect::<Vec<&i32>>();
        if negatives.iter().count() > 0 {
            panic!("negatives not allowed, received: {:?}", negatives);
        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> StringCalculator {
        StringCalculator {..Default::default()}
    }

    #[test]
    fn adding_zero_numbers_returns_zero() {
        let mut sc = init();
        assert_eq!(sc.add("".to_string()), 0);
    }

    #[test]
    fn adding_one_number() {
        let mut sc = init();
        assert_eq!(sc.add("1".to_string()), 1);
    }

    #[test]
    fn adding_two_numbers() {
        let mut sc = init();
        assert_eq!(sc.add("1,2".to_string()), 3);
    }

    #[test]
    fn adding_any_amount_of_numbers() {
        let mut sc = init();
        assert_eq!(sc.add("2,2,2".to_string()), 6);
        assert_eq!(sc.add("1,1,1,1,1,1,1".to_string()), 7);
    }

    #[test]
    fn adding_numbers_with_newline_separator() {
        let mut sc = init();
        assert_eq!(sc.add("1\n2,3".to_string()), 6);
    }

    #[test]
    fn changing_the_delimiter() {
        let mut sc = init();
        assert_eq!(sc.add("//;\n1;2".to_string()), 3);
    }

    #[test]
    #[should_panic(expected="negatives not allowed, received: [-1]")]
    fn throws_exception_on_negative_number() {
        let mut sc = init();
        sc.add("1,-1".to_string());
    }
    
    #[test]
    #[should_panic(expected="negatives not allowed, received: [-1, -2, -3]")]
    fn throws_exception_on_multiple_negative_numbers() {
        let mut sc = init();
        sc.add("1,-1,2,-2,-3,4".to_string());
    } 
    
    #[test]
    fn should_record_the_number_of_calls_to_add() {
        let mut sc = init();
        assert_eq!(sc.get_called_count(), 0);
        sc.add("1,2".to_string());
        assert_eq!(sc.get_called_count(), 1);
        sc.add("//;\n1;2".to_string());
        assert_eq!(sc.get_called_count(), 2);
    }
    
    #[test]
    fn should_ignore_numbers_greater_than_1000() {
        let mut sc = init();
        assert_eq!(sc.add("1,1000".to_string()), 1001);
        assert_eq!(sc.add("1,1001".to_string()), 1);
    }
    
    
}
