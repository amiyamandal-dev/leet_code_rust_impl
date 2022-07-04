pub struct Solution;

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut final_result: Vec<Vec<i32>> = vec![vec![]; people.len()];
        let mut sorted_vec = people.clone();
        sorted_vec.sort_by(|a, b| b[0].cmp(&a[0]));

        for i in sorted_vec.iter() {
            let mut count = i[1];
            for j in 0..final_result.len() {
                if count == 0 {
                    if final_result[j].len() == 0 {
                        final_result[j] = i.to_vec();
                        break;
                    }
                } else {
                    if final_result[j].len() != 0 {
                        if final_result[j][0] >= i[0] {
                            count -= 1;
                        }
                    } else {
                        count -= 1;
                    }
                }
            }
        }

        final_result
    }
}
