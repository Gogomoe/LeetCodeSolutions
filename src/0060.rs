impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let mut numbers: Vec<i32> = (1..=n).collect();
        let mut factorial = Vec::new();
        factorial.push(1);
        factorial.push(1);
        for i in 2..=n {
            factorial.push(i * (*factorial.last().unwrap()));
        }

        let mut result = String::new();
        k -= 1;
        for i in 0..n {
            result += &numbers.remove((k / factorial[(n - i - 1) as usize]) as usize).to_string();
            k %= factorial[(n - i - 1) as usize];
        }

        result
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::get_permutation(3, 3)); // 213
    println!("{}", Solution::get_permutation(4, 9)); // 2314
    println!("{}", Solution::get_permutation(3, 1)); // 123
}