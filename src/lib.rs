use core::panic;
use std::collections::{HashMap, LinkedList};

pub struct Rpn {
    priority: HashMap<String, i32>,
}

impl Rpn {
    pub fn new() -> Rpn {
        let priority: HashMap<String, i32> = HashMap::from([
            ("^".to_string(), 3),
            ("+".to_string(), 1),
            ("-".to_string(), 1),
            ("*".to_string(), 2),
            ("/".to_string(), 2),
        ]);
        Rpn { priority }
    }

    pub fn solve(&self, equation: String) -> Option<f64> {
        if !Self::check_brackets(&equation) {
            return None;
        }
        if let Some(split_str) = self.split(equation) {
            let tokens = self.convert_to_rpn(split_str);
            if let Some(result) = self.evaluate_rpn(&tokens) {
                return Some(result);
            } else {
                return None;
            }
        }
        None
    }

    fn check_brackets(equation: &String) -> bool {
        let mut bracket_counter: i32 = 0;
        for c in equation.chars() {
            if c == '(' {
                bracket_counter += 1;
            } else if c == ')' {
                bracket_counter -= 1;
            }

            if bracket_counter < 0 {
                println!("Wrong brackets");
                return false;
            }
        }
        if bracket_counter > 0 {
            println!("Wrong brackets");
            return false;
        }
        true
    }

    fn get_priority(&self, key: &str) -> i32 {
        if let Some(prio) = self.priority.get(key) {
            return *prio;
        }
        0
    }

    fn operation(a: f64, b: f64, op: &str) -> Option<f64> {
        match op {
            "+" => Some(b + a),
            "-" => Some(b - a),
            "*" => Some(b * a),
            "/" => Some(b / a),
            "^" => Some(b.powf(a)),
            _ => None,
        }
    }

    fn flush_buffer(buffer: &mut String, result: &mut Vec<String>) {
        if !buffer.is_empty() {
            result.push(buffer.clone());
            buffer.clear();
        }
    }

    fn split(&self, equation: String) -> Option<Vec<String>> {
        let mut buffer = String::new();
        let mut result = Vec::new();
        let mut prev_char: char = 0 as char;
        for c in equation.chars() {
            if c.is_ascii_digit() || c == '.' {
                buffer.push(c);
            } else if c == '(' || c == ')' {
                Self::flush_buffer(&mut buffer, &mut result);
                result.push(String::from(c));
            } else if self.priority.contains_key(&String::from(c)) {
                if self.priority.contains_key(&String::from(prev_char)) {
                    return None;
                }
                if c == '-' && buffer.is_empty() {
                    result.extend(["0".to_string(), "-".to_string()]);
                } else {
                    Self::flush_buffer(&mut buffer, &mut result);
                    result.push(String::from(c));
                }
            }

            prev_char = c;
        }
        Self::flush_buffer(&mut buffer, &mut result);
        Some(result)
    }

    fn evaluate_rpn(&self, rpn: &Vec<String>) -> Option<f64> {
        let mut s: LinkedList<f64> = LinkedList::new();
        for token in rpn {
            if self.priority.contains_key(token) {
                let a = s.pop_back().unwrap();
                let b = s.pop_back().unwrap();
                if let Some(result) = Self::operation(a, b, token) {
                    s.push_back(result);
                } else {
                    return None;
                }
            } else {
                s.push_back(token.parse::<f64>().unwrap());
            }
        }
        if s.len() != 1 {
            return None;
        }
        Some(*s.back().unwrap())
    }

    fn convert_to_rpn(&self, tokens: Vec<String>) -> Vec<String> {
        let mut output_list: Vec<String> = Vec::new();

        let mut s = LinkedList::new();

        for token in tokens {
            if token.parse::<f64>().is_ok() {
                output_list.push(token);
            } else if token == "(" {
                s.push_back(token);
            } else if token == ")" {
                while !s.is_empty() && s.back().unwrap() != "(" {
                    output_list.push(s.pop_back().unwrap());
                }
                s.pop_back();
            } else if let Some((op, prio)) = self.priority.get_key_value(&token) {
                while !s.is_empty() && self.get_priority(s.back().unwrap()) >= *prio {
                    output_list.push(s.pop_back().unwrap());
                }
                s.push_back(op.to_string());
            }
        }
        while !s.is_empty() {
            output_list.push(s.pop_back().unwrap());
        }
        output_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_rpn_basic_1() {
        let calc = Rpn::new();
        let s = vec![
            "3".to_string(),
            "^".to_string(),
            "4".to_string(),
            "+".to_string(),
            "(".to_string(),
            "11".to_string(),
            "-".to_string(),
            "(".to_string(),
            "3".to_string(),
            "*".to_string(),
            "2".to_string(),
            ")".to_string(),
            ")".to_string(),
            "/".to_string(),
            "2".to_string(),
        ];
        assert_eq!(
            calc.convert_to_rpn(s),
            vec!["3", "4", "^", "11", "3", "2", "*", "-", "2", "/", "+"]
        );
    }

    #[test]
    fn evaluate_rpn_basic_1() {
        let calc = Rpn::new();
        let rpn = vec![
            String::from("5"),
            String::from("3"),
            String::from("+"),
            String::from("6"),
            String::from("2"),
            String::from("/"),
            String::from("*"),
            String::from("3"),
            String::from("5"),
            String::from("*"),
            String::from("+"),
        ];
        assert_eq!(calc.evaluate_rpn(&rpn), Some(39.0));
    }

    #[test]
    #[should_panic]
    fn unsopported_operation() {
        let calc = Rpn::new();
        let equation = String::from("( 2 % 2 ) / 3 * 0.33");
        assert_eq!(calc.solve(equation), Some(0.44));
    }

    #[test]
    fn wrong_brackets() {
        let calc = Rpn::new();
        let equation = String::from("( 2 % 2 )) / 3 * 0.33");
        assert_eq!(calc.solve(equation), None);
    }

    #[test]
    fn solve_basic_1() {
        let calc = Rpn::new();
        let equation = String::from("( 2 + 2 ) / 3 * 0.33");
        assert_eq!(calc.solve(equation), Some(0.44));
    }

    #[test]
    fn solve_basic_2() {
        let calc = Rpn::new();
        let equation = String::from("2 ^ 3");
        assert_eq!(calc.solve(equation), Some(8.0));
    }

    #[test]
    fn solve_basic_3() {
        let calc = Rpn::new();
        let equation = String::from("3 / 2");
        assert_eq!(calc.solve(equation), Some(1.5));
    }

    #[test]
    fn solve_basic_4() {
        let calc = Rpn::new();
        let equation = String::from("3 - 2");
        assert_eq!(calc.solve(equation), Some(1.0));
    }

    #[test]
    fn solve_basic_6() {
        let calc = Rpn::new();
        let equation = String::from("(2)+(-2)");
        assert_eq!(calc.solve(equation), Some(0.0));
    }

    #[test]
    fn solve_basic_7() {
        let calc = Rpn::new();
        let equation = String::from("2 + -2");
        assert_eq!(calc.solve(equation), Some(0.0));
    }

    #[test]
    fn solve_basic_8() {
        let calc = Rpn::new();
        let equation = String::from("-2 + 2");
        assert_eq!(calc.solve(equation), Some(0.0));
    }

    #[test]
    fn solve_basic_9() {
        let calc = Rpn::new();
        let equation = String::from("2 * (-2)");
        assert_eq!(calc.solve(equation), Some(-4.0));
    }

    #[test]
    fn solve_basic_10() {
        let calc = Rpn::new();
        let equation = String::from("2 * (-(-2))");
        assert_eq!(calc.solve(equation), Some(4.0));
    }

    #[test]
    fn solve_basic_11() {
        let calc = Rpn::new();
        let equation = String::from("-2 * (-(-2))");
        assert_eq!(calc.solve(equation), Some(-4.0));
    }

    #[test]
    fn wrong_equation_two_operation_next_to_each_other() {
        let calc = Rpn::new();
        let equation = String::from("2 *-(-(-2))");
        assert_eq!(calc.solve(equation), None);
    }

    #[test]
    fn split_basic_1() {
        let calc = Rpn::new();
        let equation = String::from("3 / 2");
        calc.split(equation);
    }
}
