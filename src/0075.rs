impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut pos = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                nums.swap(pos, i);
                pos += 1;
            }
        }
        let mut pos = nums.len() - 1;
        for i in (0..nums.len()).rev() {
            if nums[i] == 2 {
                nums.swap(pos, i);
                pos -= 1;
            }
        }
    }
}

struct Solution {}

fn main() {
    let mut vec1 = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut vec1);
    println!("{:?}", vec1); // [0,0,1,1,2,2]

    let mut vec2 = vec![2, 0, 1];
    Solution::sort_colors(&mut vec2);
    println!("{:?}", vec2); // [0,1,2]

    let mut vec3 = vec![0];
    Solution::sort_colors(&mut vec3);
    println!("{:?}", vec3); // [0]

    let mut vec4 = vec![1];
    Solution::sort_colors(&mut vec4);
    println!("{:?}", vec4); // [1]
}