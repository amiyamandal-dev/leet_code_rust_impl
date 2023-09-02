pub struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut nums_t = nums;
        nums_t.sort_by(|a, b| {
            let temp_a = a.pow(2);
            let temp_b = b.pow(2);
            temp_a.partial_cmp(&temp_b).unwrap()
        });
        nums_t.into_iter().map(|x| x.pow(2)).collect()
    }
}
