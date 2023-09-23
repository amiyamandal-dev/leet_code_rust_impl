pub struct Solution;

fn back_tracking_str(result:&mut Vec<String>, string_val:&mut String, open:usize, close:usize, n:usize){
    if string_val.len() == 2*n{
        result.push(string_val.to_string());
        return;
    } 
    if open < n{
        string_val.push('(');
        back_tracking_str(result, string_val, open+1, close, n);
        string_val.pop();
    }
    if close < open{
        string_val.push(')');
        back_tracking_str(result, string_val, open, close+1, n);
        string_val.pop();

    }
    
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result:Vec<String> = Vec::new();
        let mut temp_str:String = String::new();
        back_tracking_str(&mut result, &mut temp_str, 0, 0, n as usize);
        result
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let rez = Solution::generate_parenthesis(3);
        println!("{:?}", rez);
    }
}
