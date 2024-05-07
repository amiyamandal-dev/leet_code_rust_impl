struct Solution;
impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut _temp: Vec<i32> = Vec::with_capacity(nums.len() * 2);
        _temp.extend(nums.clone());
        _temp.extend(nums);
        _temp
    }
}
