impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let board1 = board.clone();
        for i in 0..board1.len(){
            for j in 0..board1[0].len(){
                let mut count = 0;
                if i > 0{
                    if j > 0{
                        count += board1[i-1][j-1];
                    }
                    if j < board1[0].len() -1 {
                        count += board1[i-1][j+1];
                    }
                    count += board1[i-1][j];
                }
                if i < board1.len() - 1 {
                    if j > 0{
                        count += board1[i+1][j-1];
                    }
                    if j < board1[0].len() -1 {
                        count += board1[i+1][j+1];
                    }
                    count += board1[i+1][j];
                }
                if j > 0{
                    count += board1[i][j-1];
                }
                if j < board1[0].len() -1 {
                    count += board1[i][j+1];
                } 
                if count < 2 || count > 3{
                    board[i][j] = 0;
                }else if count == 3{
                    board[i][j] = 1;
                }
            }
        }
    }
}