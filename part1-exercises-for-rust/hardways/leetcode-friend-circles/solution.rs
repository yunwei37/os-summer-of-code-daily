impl Solution {
    fn deep(m: &Vec<Vec<i32>>,a:&mut Vec<i32>,i:usize){
            for j in 0..m.len(){
                if m[i][j] == 1 && a[j] == 0{
                    a[j] = 1;
                    Solution::deep(m,a,j);
                }
            }
    }

    pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
        let mut a = vec![0;m.len()];
        let mut count = 0;
        for i in 0..m.len(){
            if a[i] ==0 {
                count= count+1;
                Solution::deep(&m,&mut a,i);
            }
        }
        count
    }
}