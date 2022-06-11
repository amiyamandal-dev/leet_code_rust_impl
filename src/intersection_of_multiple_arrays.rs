pub struct Solution;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let t_vec = nums[0].clone();
        let mut result: Vec<i32> = Vec::new();
        let mut count = 0;
        for i in t_vec.into_iter() {
            for v in nums.iter() {
                if v.contains(&i) {
                    count += 1;
                }
            }
            if count == nums.len() {
                result.push(i);
            }
            count = 0;
        }

        result.sort();
        result
    }
}
