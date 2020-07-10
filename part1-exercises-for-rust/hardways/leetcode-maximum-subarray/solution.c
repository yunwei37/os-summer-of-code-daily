int maxSubArray(int* nums, int numsSize){
    int subsum = 0, maxsum = -2147483648;
    
    for(int i = 0; i < numsSize; i ++)
    {
        subsum += nums[i];
        if(subsum > maxsum)
        {
            maxsum = subsum;
        }
        if(subsum < 0) subsum = 0;
    }
    return maxsum;
}