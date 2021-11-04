impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result = Vec::new();
        let mut values = Vec::new();
        Solution::k_sum(4, 0, target, &mut values, &mut result, &nums);
        return result;
    }


    fn k_sum(k: usize, start: usize, target: i32, values: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, nums: &Vec<i32>) {
        let len = nums.len();
        if k > 2 {
            let mut i = start;
            while i as i32 <= (len as i32 - k as i32) {
                let value = nums[i];
                values.push(value);
                Solution::k_sum(k - 1, i + 1, target - value, values, result, nums);
                values.pop();

                i = Solution::find_next_distinct(i, nums);
            }
            return;
        }

        // k == 2
        let mut l = start;
        let mut r = nums.len() as i32 - 1;
        while (l as i32) < r {
            let value = nums[l] + nums[r as usize];
            if value > target {
                r = Solution::find_prev_distinct(r, nums);
            } else if value < target {
                l = Solution::find_next_distinct(l, nums);
            } else {
                let mut it = values.clone();
                it.push(nums[l]);
                it.push(nums[r as usize]);
                result.push(it);
                l = Solution::find_next_distinct(l, nums);
                r = Solution::find_prev_distinct(r, nums);
            }
        }
    }

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
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0)); // [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
    println!("{:?}", Solution::four_sum(vec![2, 2, 2, 2, 2], 8)); // [[2,2,2,2]]
    println!("{:?}", Solution::four_sum(vec![0], 0)); // []
}