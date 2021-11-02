impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        fn find_next_distinct(mut i: usize, nums: &Vec<i32>) -> usize {
            let value = nums[i];
            while i < nums.len() && nums[i] == value {
                i += 1;
            }
            return i;
        }
        fn find_prev_distinct(mut i: i32, nums: &Vec<i32>) -> i32 {
            let value = nums[i as usize];
            while i >= 0 && nums[i as usize] == value {
                i -= 1;
            }
            return i;
        }
        let mut result = Vec::new();
        let mut i = 0;
        while i < nums.len() && nums[i] <= 0 {
            let mut j = i + 1;
            let mut k = (nums.len() - 1) as i32;
            while (j as i32) < k {
                let sum = nums[i] + nums[j] + nums[k as usize];
                if sum > 0 {
                    k = find_prev_distinct(k, &nums);
                } else if sum < 0 {
                    j = find_next_distinct(j, &nums);
                } else {
                    result.push(vec![nums[i], nums[j], nums[k as usize]]);
                    k = find_prev_distinct(k, &nums);
                    j = find_next_distinct(j, &nums);
                }
            }
            i = find_next_distinct(i, &nums);
        }

        result
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
}