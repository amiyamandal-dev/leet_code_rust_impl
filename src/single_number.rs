pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut mapping: HashMap<i32, i32> = HashMap::new();

        for i in nums.iter() {
            let rez = match mapping.get(i) {
                Some(t) => *t + 1,
                None => 1,
            };
            mapping.insert(*i, rez);
        }

        for (key, val) in mapping.iter() {
            if *val == 1 {
                return *key;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(1, Solution::single_number(vec![2, 2, 1]));
    }
}
