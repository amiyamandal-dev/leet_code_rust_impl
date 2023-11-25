use std::collections::HashMap;

struct Solution{}

impl Solution {
    fn dfs(result_list:&mut Vec<String>, current_string:String, number_mapping: & HashMap<String, Vec<String>>, split_digits:&Vec<String>, index_val:usize){
        if current_string.len() == split_digits.len(){
            result_list.push(current_string);
            return;
        } else {
            let mapped_val = match number_mapping.get(&split_digits[index_val]) {
                Some(t) => t,
                None => panic!("wrong key"),
            };
            for i in mapped_val.into_iter(){
                let new_string = current_string.clone() + i;
                Solution::dfs(result_list, new_string, number_mapping, split_digits, index_val +1);
            }
        }

    }
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result_list:Vec<String> = Vec::new();

        if digits.is_empty(){
            return result_list;
        }


        let mut number_mapping:HashMap<String, Vec<String>> = HashMap::new();
        number_mapping.insert("2".to_string(), vec!["a".to_string(), "b".to_string(),"c".to_string()]);
        number_mapping.insert("3".to_string(), vec!["d".to_string(), "e".to_string(),"f".to_string()]);
        number_mapping.insert("4".to_string(), vec!["g".to_string(), "h".to_string(),"i".to_string()]);
        number_mapping.insert("5".to_string(), vec!["j".to_string(), "k".to_string(),"l".to_string()]);
        number_mapping.insert("6".to_string(), vec!["m".to_string(), "n".to_string(),"o".to_string()]);
        number_mapping.insert("7".to_string(), vec!["p".to_string(), "q".to_string(),"r".to_string(), "s".to_string()]);
        number_mapping.insert("8".to_string(), vec!["t".to_string(), "u".to_string(),"v".to_string()]);
        number_mapping.insert("9".to_string(), vec!["w".to_string(), "x".to_string(),"y".to_string(), "z".to_string()]);

        let split_digits:Vec<String> = digits.chars().map(|x|{x.to_string()}).collect();
        

        Solution::dfs(&mut result_list, "".to_owned(), &number_mapping, &split_digits, 0);
        result_list

    }
}