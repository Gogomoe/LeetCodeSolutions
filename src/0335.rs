impl Solution {
    pub fn is_self_crossing(x: Vec<i32>) -> bool {
        for i in 0..x.len() {
            if i >= 3 && x[i] >= x[i - 2] && x[i - 1] <= x[i - 3] {
                return true;
            }
            if i >= 4 && x[i - 1] == x[i - 3] && x[i] >= x[i - 2] - x[i - 4] {
                return true;
            }
            if i >= 5 && x[i] + x[i - 4] >= x[i - 2] && x[i - 2] >= x[i - 4]
                && -x[i - 1] + x[i - 3] - x[i - 5] <= 0 && -x[i - 1] + x[i - 3] >= 0 {
                return true;
            }
        }
        false
    }
}

struct Solution {}

fn main() {}