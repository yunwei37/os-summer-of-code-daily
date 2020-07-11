void deep(int** M, int MSize, int *visited, int index)
{
    for (int i = 0; i < MSize; i++) {
        if(M[index][i] == 1 && visited[i] == 0) {
            visited[i] = 1;
            deep(M, MSize, visited, i);
        }
    }
}

int findCircleNum(int** M, int MSize, int* MColSize){
    int visited[MSize];
    memset(visited, 0, sizeof(visited));
    int count = 0;
    for (int i = 0; i < MSize; i++) {
        if (visited[i] == 0) {
            deep(M, MSize, visited, i);
            count++;
        }
    }
    return count;
}