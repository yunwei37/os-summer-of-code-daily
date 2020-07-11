int i=0;

struct TreeNode {
   int val;
    struct TreeNode *left;
    struct TreeNode *right;
};

void inorder(struct TreeNode* root,int* array){
    if(!root) return;
    inorder(root->left,array);
    array[i++] = root->val;
    inorder(root->right,array);
}

int* inorderTraversal(struct TreeNode* root, int* returnSize){
    int *a = malloc(sizeof(int)*10000);
    i = 0;
    inorder(root,a);
    *returnSize = i;
    return a;
}