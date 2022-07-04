pub struct Solution;

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut new_matrix = transpose(matrix);
        for i in 0..matrix.len() {
            new_matrix[i].reverse();
            matrix[0] = new_matrix[i].clone();
        }
    }
}
