impl Solution {
    pub fn find_best_value(mut arr: Vec<i32>, target: i32) -> i32 {
        arr.sort();

        let mut part_sum = 0;
        let mut right_index = arr.len();
        for i in 0..arr.len() {
            let sum = part_sum + arr[i] * (arr.len() - i) as i32;
            if sum > target {
                right_index = i;
                break;
            }
            part_sum += arr[i];
        }

        if right_index >= arr.len() - 1 {
            return arr[arr.len() - 1];
        }

        let remain_elements = (arr.len() - right_index) as i32;
        let result = (target - part_sum) / (remain_elements);
        if ((result + 1) * remain_elements + part_sum - target).abs() <
            (result * remain_elements + part_sum - target).abs()
        {
            return result + 1;
        }

        result
    }
}

struct Solution {}

fn main() {}