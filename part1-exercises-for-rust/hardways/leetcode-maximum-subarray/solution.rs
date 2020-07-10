impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max = 0;
        let mut maxi = nums[0];
        for i in nums{
            if sum + i < 0{
                sum = 0;
            }else{
                sum = sum + i;
                if sum > max  {
                    max = sum;
                }
            }
            if i > maxi {
                maxi =i;
            }
        }
        if maxi < 0{
            max = maxi;
        }
        max
    }
}