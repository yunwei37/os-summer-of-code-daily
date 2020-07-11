struct TreeNode {
   int val;
    struct TreeNode *left;
    struct TreeNode *right;
};

int maxDepth(struct TreeNode* root){
    if(root==0) 
        return 0;
    int height = 1;
    if(root->left)
        height += maxDepth(root->left);
    if(root->right){
        int a =  maxDepth(root->right)+1;
        height = a > height? a:height;
    }
    return height;
}