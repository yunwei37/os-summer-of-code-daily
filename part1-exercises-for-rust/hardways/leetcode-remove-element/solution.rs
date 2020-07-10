impl Solution {
    pub fn remove_element(mut nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index = 0;
        let mut count = 0;
        let mut a = Vec::<i32>::new();
        for i in 0..nums.len(){
            if nums[i] != val {
                nums[index] =  nums[i].clone();
                index = index+1;
                count = count+1;
            }
        }
        count
    }
}