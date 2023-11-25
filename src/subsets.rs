struct Solution{}

impl Solution {
     
     fn dfs(index_val:usize, subset_val:&mut Vec<i32>, result_val:&mut Vec<Vec<i32>>, nums: &Vec<i32>) {
        if index_val == nums.len() {
            result_val.push(subset_val.clone());
            return;
        }

        // Include the current element in the subset
        subset_val.push(nums[index_val]);
        Solution::dfs(index_val + 1, subset_val, result_val, nums);

        // Exclude the current element from the subset
        subset_val.pop();
        Solution::dfs(index_val + 1, subset_val, result_val, nums);
     }


    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result_val:Vec<Vec<i32>> = Vec::new();
        let mut subset_val:Vec<i32> = Vec::new();
        Solution::dfs(0, &mut subset_val, &mut result_val, &nums);
        return result_val;
    }
}



#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let t = vec![1,2,3];
        let rez = Solution::subsets(t.clone());
        assert_eq!(2usize.pow(t.len() as u32), rez.len());
    }
}
