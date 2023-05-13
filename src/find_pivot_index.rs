pub struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut total_sum: i32 = nums.iter().sum();
        let mut left_sum = 0;
        for (index_val, elemt) in nums.iter().enumerate() {
            let mut right_sum = total_sum - nums[index_val] - left_sum;
            if left_sum == right_sum {
                return index_val as i32;
            }
            left_sum += *elemt;
        }
        -1
    }
}
