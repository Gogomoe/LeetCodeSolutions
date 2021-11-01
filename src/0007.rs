impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let neg = x < 0;
        if x < 0 {
            if x == i32::MIN {
                return 0;
            }
        }
        x = x.abs();

        let mut result = 0;
        while x != 0 {
            if result > i32::MAX / 10 { return 0; }
            result *= 10;
            if result > i32::MAX - x % 10 { return 0; }
            result += x % 10;
            x /= 10;
        }

        return result * if neg { -1 } else { 1 };
    }
}

struct Solution {}

fn main() {
    println!("{}", Solution::reverse(123));
    println!("{}", Solution::reverse(-123));
    println!("{}", Solution::reverse(120));
    println!("{}", Solution::reverse(0));
}