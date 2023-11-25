struct Solution{}

impl Solution {

    fn dfs(target:i32, total_sum:i32,temp_list:&mut Vec<i32>, result_list:&mut Vec<Vec<i32>>, index_val:usize, candidates: & Vec<i32>){
        if target == total_sum{
            result_list.push(temp_list.to_vec());
        } else if total_sum > target {
            return;
        } else if total_sum < target && index_val < candidates.len(){
            temp_list.push(candidates[index_val]);
            Solution::dfs(target, total_sum + candidates[index_val], temp_list, result_list, index_val, candidates);
            temp_list.pop();
            Solution::dfs(target, total_sum , temp_list, result_list, index_val + 1, candidates);

        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result_list:Vec<Vec<i32>> = Vec::new();
        let mut temp_list:Vec<i32> = Vec::new();
        Solution::dfs(target, 0, &mut temp_list, &mut result_list, 0, &candidates);
        result_list
        
    }
}