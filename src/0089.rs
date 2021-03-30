impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut res = Vec::new();
        res.push(0);
        res.push(1);

        let mut bit = 1;
        for _ in 1..n {
            bit <<= 1;
            let size = res.len();

            for i in (0..size).rev() {
                let it = res[i];
                res.push(it | bit);
            }
        }

        res
    }
}

struct Solution {}

fn main() {}