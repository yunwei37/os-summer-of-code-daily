impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut a = Vec::<(usize,usize)>::new();
        for i in 0..matrix.len(){
            for j in 0..matrix[0].len(){
                if matrix[i][j] == 0{
                    a.push((i,j));
                }
            }
        }
        for m in a{
            for k in 0..matrix.len(){
                matrix[k][m.1] = 0;
            }
            for k in 0..matrix[0].len(){
                matrix[m.0][k] = 0;
            }
        }
    }
}

// 事实上这个的时间复杂度挺高的，但由于0比较稀疏，可能在实际使用中表现较好