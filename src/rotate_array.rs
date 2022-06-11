pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let temp_nums = nums.clone();
        let len_val = nums.len();
        for i in 0..len_val {
            nums[(k as usize + i) % len_val] = temp_nums[i];
        }
    }
}
