impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() { return vec![-1, -1]; }
        let i = Solution::binary_search_first(&nums, target);
        if i == -1 { return vec![-1, -1]; }
        let j = Solution::binary_search_last(&nums, target);
        return vec![i, j];
    }

    fn binary_search_first(nums: &Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] >= target {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        if nums[l] != target {
            return -1;
        }
        return l as i32;
    }

    fn binary_search_last(nums: &Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let mid = (l + r + 1) / 2;
            if nums[mid] <= target {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        if nums[l] != target {
            return -1;
        }
        return l as i32;
    }
}

struct Solution {}

fn main() {
    println!("{:?}", Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)); // [3,4]
    println!("{:?}", Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6)); // [-1,-1]
    println!("{:?}", Solution::search_range(vec![], 0)); // [-1,-1]
    println!("{:?}", Solution::search_range(vec![1], 1)); // [0,0]
}