impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut result = 0;
        let mut number = 0;
        let mut sign = 1;

        for char in s.chars() {
            match char {
                '(' => {
                    stack.push(result);
                    stack.push(sign);
                    result = 0;
                    sign = 1;
                }
                ')' => {
                    result += sign * number;
                    number = stack.pop().unwrap() * result;
                    result = stack.pop().unwrap();
                    sign = 1;
                }
                '+' => {
                    result += sign * number;
                    number = 0;
                    sign = 1;
                }
                '-' => {
                    result += sign * number;
                    number = 0;
                    sign = -1;
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                    number = number * 10 + (char as u8 - '0' as u8) as i32;
                }
                _ => {}
            }
        }
        if number != 0 {
            result += sign * number;
        }
        return result;
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::calculate("1 + 1".to_string())); // 2
    println!("{}", Solution::calculate(" 2-1 + 2 ".to_string())); // 3
    println!("{}", Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string())); // 23
}