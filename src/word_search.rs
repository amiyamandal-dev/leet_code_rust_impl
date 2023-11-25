use std::collections::HashSet;

struct Solution{}

impl Solution {

    fn back_track(board: &Vec<Vec<char>>, current_row:i32, current_col:i32,index_of_word:usize, char_word:&Vec<char>, 
        path_visited:&mut HashSet<[i32; 2]>, num_rows:usize, num_col:usize) -> bool{
            let temp_path:[i32;2] = [current_row, current_col];
            if index_of_word == char_word.len(){
                return true;
            } else if current_row < 0 || current_col < 0 {
                return false;
            } else if current_col >= (num_col as i32) || current_row >= (num_rows as i32) {
                return false;
            } else if char_word[index_of_word] != board[current_row as usize][current_col as usize]{
                return false;
            } else if path_visited.contains(&temp_path){
                return false;
            }
            path_visited.insert(temp_path);
            // print!("{:?}", path_visited);
            let response = (Solution::back_track(board, current_row+1, current_col, index_of_word + 1, char_word, path_visited, num_rows, num_col) || 
            Solution::back_track(board, current_row-1, current_col, index_of_word + 1, char_word, path_visited, num_rows, num_col )||
            Solution::back_track(board, current_row, current_col+1, index_of_word + 1, char_word, path_visited, num_rows, num_col)||
            Solution::back_track(board, current_row, current_col-1, index_of_word + 1, char_word, path_visited, num_rows, num_col));
            path_visited.remove(&temp_path);
            response

    }


    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let num_rows = board.len();
        let num_col = board[0].len();
        let mut path_visited:HashSet<[i32; 2]> = HashSet::new();
        let char_word: Vec<char> = word.chars().collect();
        for i in 0..num_rows{
            for j in 0..num_col{
                let resp = Solution::back_track(&board, i as i32, j as i32, 0, &char_word, &mut path_visited, num_rows, num_col);
                if resp == true{
                    return true;
                }
            }
        }
        false
    }
}

