pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = 0;
        for i in 0..(nums.len() - 1) {
            if nums[i] != 0 {
                let temp = nums[j];
                nums[j] = nums[i];
                nums[i] = temp;
                j += 1;
            }
        }
    }
}
