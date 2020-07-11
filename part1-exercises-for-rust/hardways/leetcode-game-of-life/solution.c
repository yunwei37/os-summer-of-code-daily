void gameOfLife(int** board, int boardSize, int* boardColSize){
    int board1[100][100];
    for(int i=0;i<boardSize;++i){
        for(int j=0;j<boardColSize[0];++j){
            board1[i][j] = board[i][j];
        }
    }
    for(int i=0;i<boardSize;++i){
        for(int j=0;j<boardColSize[0];++j){
            int count = 0;
            if( i > 0){
                    if( j > 0){
                        count += board1[i-1][j-1];
                    }
                    if (j < boardColSize[0] -1) {
                        count += board1[i-1][j+1];
                    }
                    count += board1[i-1][j];
            }
            if (i < boardSize - 1) {
                    if (j > 0){
                        count += board1[i+1][j-1];
                    }
                    if (j < boardColSize[0] -1) {
                        count += board1[i+1][j+1];
                    }
                    count += board1[i+1][j];
            }
            if (j > 0){
                    count += board1[i][j-1];
                }
            if (j < boardColSize[0] -1) {
                    count += board1[i][j+1];
                } 
            if (count < 2 || count > 3){
                    board[i][j] = 0;
                }else if (count == 3){
                    board[i][j] = 1;
                }
        }
    }
}