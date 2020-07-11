void setZeroes(int** matrix, int matrixSize, int* matrixColSize){
    int i, j, k;
    int rRow = -1;
    for (i = 0; i < matrixSize; i++) {
        for (j = 0; j < matrixColSize[i]; j++) {
            if (matrix[i][j] == 0) {
                for (k = 0; k < matrixColSize[i]; k++) {
                    if (matrix[i][k] == 0) {
                        matrix[i][k] = 1;
                    } else {
                        matrix[i][k] = 0;
                    }
                }
                if (rRow != -1) {
                    for (k = 0; k < matrixColSize[rRow]; k++) {
                        matrix[i][k] = matrix[rRow][k] | matrix[i][k];
                        matrix[rRow][k] = 0;
                    }
                }
                rRow = i;
                break;
            }
        }
    }
    if (rRow != -1) {
        for (j = 0; j < matrixColSize[rRow]; j++) {
            if (matrix[rRow][j] == 1) {
                for (k = 0; k < matrixSize; k++) {
                    matrix[k][j] = 0;
                }
            }
        }
    }
}