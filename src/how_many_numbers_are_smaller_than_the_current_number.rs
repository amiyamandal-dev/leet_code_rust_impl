pub struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut final_result:Vec<i32> = Vec::new();

        for (index_i, i) in nums.iter().enumerate(){
            let mut small_count = 0;
            for (index_j, j) in nums.iter().enumerate(){
                if index_i != index_j{
                    if *i > *j {
                        small_count += 1;
                    }
                }
            }
            final_result.push(small_count);
        }

        final_result
    }
}