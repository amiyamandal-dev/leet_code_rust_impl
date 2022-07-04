pub struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum_result: Vec<i32> = vec![];
        for i in 0..nums.len() {
            let mut temp_sum = 0;
            for j in 0..(i + 1) {
                temp_sum += nums[j];
            }
            sum_result.push(temp_sum);
        }
        sum_result
    }
}
