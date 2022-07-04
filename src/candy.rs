pub struct Solution;

impl Solution {
    // pub fn candy(ratings: Vec<i32>) -> i32 {
    //     if ratings.len() < 1 {
    //         return 0;
    //     }
    //     let mut left_to_right: Vec<i32> = Vec::new();
    //     left_to_right.push(1);
    //     let mut old_temp = ratings[0];
    //     for i in ratings[1..].iter() {
    //         if old_temp < *i {
    //             let last_val_updated = match left_to_right.last() {
    //                 Some(t) => *t + 1,
    //                 None => 1,
    //             };
    //             left_to_right.push(last_val_updated);
    //         } else {
    //             left_to_right.push(1);
    //         }
    //         old_temp = *i;
    //     }

    //     let mut temp_ratings = ratings.clone();
    //     temp_ratings.reverse();

    //     let mut right_to_left: Vec<i32> = Vec::new();
    //     right_to_left.push(1);
    //     old_temp = temp_ratings[0];

    //     for i in temp_ratings[1..].iter() {
    //         if old_temp < *i {
    //             let last_val_updated = match right_to_left.last() {
    //                 Some(t) => *t + 1,
    //                 None => 1,
    //             };
    //             right_to_left.push(last_val_updated);
    //         } else {
    //             right_to_left.push(1);
    //         }
    //         old_temp = *i;
    //     }

    //     right_to_left.reverse();

    //     let final_vec = left_to_right.iter().zip(right_to_left.iter());
    //     let mut result_vec = vec![];
    //     for (i, (x, y)) in final_vec.enumerate() {
    //         println!("{}: ({}, {})", i, x, y);
    //         let max_val = *x.max(y);
    //         result_vec.push(max_val);
    //     }

    //     result_vec.iter().sum()
    // }

    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.len() < 1 {
            return 0;
        }
        let mut temp_ratings = ratings.clone();
        temp_ratings.reverse();

        let mut left_to_right: Vec<i32> = Vec::new();
        left_to_right.push(1);

        let mut right_to_left: Vec<i32> = Vec::new();
        right_to_left.push(1);
        let it = ratings[1..].iter().zip(temp_ratings[1..].iter());
        let mut old_1 = ratings[0];
        let mut old_2 = temp_ratings[0];
        for (x, y) in it {
            if old_1 < *x {
                let last_val_updated = match left_to_right.last() {
                    Some(t) => *t + 1,
                    None => 1,
                };
                left_to_right.push(last_val_updated);
            } else {
                left_to_right.push(1);
            }
            old_1 = *x;

            if old_2 < *y {
                let last_val_updated = match right_to_left.last() {
                    Some(t) => *t + 1,
                    None => 1,
                };
                right_to_left.push(last_val_updated);
            } else {
                right_to_left.push(1);
            }
            old_2 = *y;
        }
        right_to_left.reverse();

        let final_vec = left_to_right.iter().zip(right_to_left.iter());
        let mut result_vec = vec![];
        for (i, (x, y)) in final_vec.enumerate() {
            println!("{}: ({}, {})", i, x, y);
            let max_val = *x.max(y);
            result_vec.push(max_val);
        }

        result_vec.iter().sum()
    }
}
