use std::collections::HashMap;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut a = HashMap::new();
        let mut max = 0;
        for i in nums {
            if let Some(n1) =  a.get(&(i)){
                continue;
            }
            let mut n = 0;
            let mut m = 0;
            let mut sum = 0;
            if let Some(n1) =  a.get(&(i+1)){
                n = *n1;
            }
            if let Some(n1) =  a.get(&(i-1)){
                m = *n1;
            }
            sum = n+m+1;
            if max <sum{
                max = sum;
            }
            //println!("{},{},{},{}",sum,i,m,n);
            a.insert(i,sum);
            a.insert(i - m,sum);
            a.insert(i + n,sum);
        }
        max
    }
}