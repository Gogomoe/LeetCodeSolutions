impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut start_pos = len - 1;
        while start_pos > 0 && nums[start_pos - 1] >= nums[start_pos] {
            start_pos -= 1;
        }

        if start_pos == 0 {
            nums.sort();
            return;
        }

        let mut min_pos = start_pos;
        for i in start_pos..len {
            if nums[i] > nums[start_pos - 1] {
                min_pos = i;
            } else {
                break;
            }
        }

        nums.swap(start_pos - 1, min_pos);
        nums[start_pos..len].sort();
        return;
    }
}

struct Solution {}

fn main() {
    let mut vec1 = vec![1, 2, 3];
    Solution::next_permutation(&mut vec1);
    println!("{:?}", vec1); // [1, 3, 2]
    let mut vec2 = vec![3, 2, 1];
    Solution::next_permutation(&mut vec2);
    println!("{:?}", vec2); // [1, 2, 3]
    let mut vec3 = vec![1, 1, 5];
    Solution::next_permutation(&mut vec3);
    println!("{:?}", vec3); // [1, 5, 1]
    let mut vec4 = vec![1];
    Solution::next_permutation(&mut vec4);
    println!("{:?}", vec4); // [1]
}