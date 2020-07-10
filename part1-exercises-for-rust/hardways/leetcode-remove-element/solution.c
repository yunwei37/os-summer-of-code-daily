int removeElement(int* nums, int numsSize, int val){
    int size = 0;
    int i = 0;

    for (i = 0; i <= numsSize - 1; i++)
    {
        if (nums[i] != val) // ignore the same
        {
            nums[size++] = nums[i];
        }
    }

    return size;
}