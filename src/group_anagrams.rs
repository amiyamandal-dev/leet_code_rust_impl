use std::collections::HashMap;
use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() < 2 {
            return vec![strs];
        }
        let mut final_result: HashMap<String, Vec<String>> = HashMap::new();
        let mut vec_of_all_set: Vec<HashSet<char>> = Vec::new();

        for i in strs.iter() {
            if *i == "".to_string() {
                if final_result.contains_key(i) {
                    final_result.get_mut(i).unwrap().push(i.clone());
                } else {
                    let mut temp_vec = vec![];
                    temp_vec.push(i.clone());
                    final_result.insert(i.to_string(), temp_vec.to_vec());
                }
            } else {
                let temp_set: HashSet<char> = i.chars().collect();
                let mut count = 0;
                for j in vec_of_all_set.iter() {
                    if j.intersection(&temp_set).collect::<Vec<&char>>().len() == j.len() {
                        count = 1;
                        let mut chars: Vec<u8> = i.as_bytes().to_vec();
                        chars.sort_by(|a, b| b.cmp(a));
                        let s = String::from_utf8(chars).unwrap();
                        final_result.get_mut(&s).unwrap().push(i.clone());
                    }
                }
                if count == 0 {
                    let mut chars: Vec<u8> = i.as_bytes().to_vec();
                    chars.sort_by(|a, b| b.cmp(a));
                    let s = String::from_utf8(chars).unwrap();
                    let mut temp_vec = vec![];
                    vec_of_all_set.push(temp_set);
                    temp_vec.push(i.clone());
                    final_result.insert(s, temp_vec.to_vec());
                }
            }
        }

        let final_vec = final_result.values().cloned().collect::<Vec<Vec<String>>>();
        final_vec
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        // let j = Solution::group_anagrams(vec![
        //     "eat".to_string(),
        //     "tea".to_string(),
        //     "tan".to_string(),
        //     "ate".to_string(),
        //     "nat".to_string(),
        //     "bat".to_string(),
        // ]);
        let j = Solution::group_anagrams(vec!["ddddddddddg".to_string(),"dgggggggggg".to_string()]);
        println!("{:?}", j);
    }
}
