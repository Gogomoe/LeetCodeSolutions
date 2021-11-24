impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut counts = Vec::new();
        counts.push(1usize);
        for i in 1..=(n as usize) {
            let mut count = 0;
            for j in 0..i {
                count += counts[j] * counts[i - 1 - j];
            }
            counts.push(count);
        }
        return counts[n as usize] as i32;
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::num_trees(3));
    println!("{:?}", Solution::num_trees(1));
}
