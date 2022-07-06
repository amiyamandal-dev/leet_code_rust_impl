pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let super_set: HashSet<i32> = nums.clone().into_iter().collect();
        let mut longest = i32::MIN;
        for i in nums.iter() {
            if !super_set.contains(&(*i - 1)) {
                let mut length = 1;
                while super_set.contains(&(*i + length)) {
                    length += 1;
                }
                longest = longest.max(length);
            }
        }
        longest
    }
}
