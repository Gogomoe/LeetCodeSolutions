impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        let sign = dividend.signum() * divisor.signum();
        if sign == 0 {
            return 0;
        }

        let mut result = 0;
        let pos_divisor = if divisor == i32::MIN {
            return if dividend == i32::MIN { 1 } else { 0 };
        } else {
            divisor.abs()
        };
        let mut pos_dividend = if dividend == i32::MIN {
            result += 1;
            -(dividend + pos_divisor)
        } else {
            dividend.abs()
        };

        loop {
            let mut times = 1;
            let mut times_divisor = pos_divisor;
            while (times_divisor <= i32::MAX - times_divisor) && pos_dividend >= times_divisor + times_divisor {
                times_divisor += times_divisor;
                times += times;
            }

            if pos_dividend >= times_divisor {
                pos_dividend -= times_divisor;
                result += times;
            } else {
                break;
            }
        }

        return sign * result;
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::divide(10, 3)); // 3
    println!("{:?}", Solution::divide(7, -3)); // -2
    println!("{:?}", Solution::divide(0, 1)); // 0
    println!("{:?}", Solution::divide(1, 1)); // 1
    println!("{:?}", Solution::divide(2147483647, 1)); // 2147483647
}