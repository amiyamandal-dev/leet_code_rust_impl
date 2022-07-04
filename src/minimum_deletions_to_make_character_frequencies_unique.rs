pub struct Solution;

use std::collections::HashMap;

fn get_only_vec<K, V>(map_val: HashMap<K, V>) -> impl Iterator<Item = V> {
    map_val.into_iter().map(|(_k, v)| v)
}

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut temp_map: HashMap<char, i32> = HashMap::new();
        let mut diff = 0;

        for i in s.chars() {
            if temp_map.contains_key(&i) {
                let mut val = temp_map.get_mut(&i).unwrap();
                *val += 1;
            } else {
                temp_map.insert(i, 1);
            }
        }
        let mut max = std::i32::MIN;
        let mut vec_val = vec![];
        for i in get_only_vec(temp_map) {
            vec_val.push(i);
            if max < i {
                max = i;
            }
        }

        vec_val.sort_by(|a, b| b.cmp(a));
        let mut diff_ = vec_val[0];
        let mut final_result = 0;
        for i in vec_val.into_iter() {
            if i > diff_ {
                if diff_ > 0 {
                    final_result += i - diff_;
                } else {
                    final_result += i;
                }
            }
            diff_ = (diff_ - 1).min(i - 1);
        }

        final_result
    }
}

/*"accdcdadddbaadbc"
a=4
c=4
d=5
b=2
 */
