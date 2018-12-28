/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @param {number} sum
 * @return {boolean}
 */
var hasPathSum = function(root, sum) {
    const dfs = (node, sum, s) => {
        s += node.val;
        if (node.left !== null) {
            if (dfs(node.left, sum, s) === true) {
                return true;
            }
        }
        
        if (node.right !== null) {
            if (dfs(node.right, sum, s) === true) {
                return true;
            }
        }
        
        if (node.left === null && node.right === null) {
            if (sum === s) return true;
        }
        return false;
    }
    
    if (root !== null)return dfs(root, sum, 0);
    return false;
};
