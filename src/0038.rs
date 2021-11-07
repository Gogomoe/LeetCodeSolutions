impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        return Solution::say(Solution::count_and_say(n - 1));
    }

    fn say(str: String) -> String {
        let mut result = String::new();
        let chars: Vec<char> = str.chars().collect();
        let mut count = 1;
        let mut number = chars[0];

        for i in 1..chars.len() {
            if chars[i] == number {
                count += 1;
            } else {
                result += &count.to_string();
                result.push(number);

                count = 1;
                number = chars[i];
            }
        }
        result += &count.to_string();
        result.push(number);
        return result;
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::count_and_say(1)); // 1
    println!("{}", Solution::count_and_say(4)); // 1211
}