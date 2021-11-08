impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }
        let num1: Vec<u8> = num1.chars().rev().map(|it| it as u8 - '0' as u8).collect();
        let num2: Vec<u8> = num2.chars().rev().map(|it| it as u8 - '0' as u8).collect();

        let mut result = Vec::new();
        for i in 0..num2.len() {
            for j in 0..num1.len() {
                if i + j == result.len() {
                    result.push(0);
                }
                result[i + j] += (num2[i] * num1[j]) as u32;
            }
        }

        let mut i = 0;
        while i < result.len() {
            let next = result[i] / 10;
            result[i] %= 10;
            if next != 0 {
                if i == result.len() - 1 {
                    result.push(0);
                }
                result[i + 1] += next;
            }
            i += 1;
        }

        return result.iter_mut().rev().map(|it| ('0' as u8 + *it as u8) as char).collect();
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::multiply("2".to_string(), "3".to_string())); // 6
    println!("{}", Solution::multiply("123".to_string(), "456".to_string())); // 56088
    println!("{}", Solution::multiply("9133".to_string(), "0".to_string())); // 0
}