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
 * @return {number[][]}
 */
var pathSum = function(root, sum) {
    let paths = [];
    const dfs = (node, sum, s, path) => {
        s += node.val;
        path.push(node.val);
        if (node.left !== null) {
            dfs(node.left, sum, s, path);
        }

        if (node.right !== null) {
            dfs(node.right, sum, s, path);
        }

        if (node.left === null && node.right === null) {
            if (sum === s) paths.push(path.slice());
        }
        path.pop();
        return false;
    }

    if (root !== null)dfs(root, sum, 0, []);
    return paths;
};
