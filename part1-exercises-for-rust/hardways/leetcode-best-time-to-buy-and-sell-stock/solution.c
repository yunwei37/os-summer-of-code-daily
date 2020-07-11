int maxProfit(int* prices, int pricesSize){
    int sum = 0;
    int max = 0;
    for(int i=0;i<pricesSize-1;++i){
        prices[i] = prices[i+1] - prices[i];
    }
    for(int i=0;i<pricesSize-1;++i){
        sum = sum + prices[i];
        if(sum < 0){
            sum = 0;
        }else if(sum>max) {
            max = sum;
        }
    }
    return max;
}