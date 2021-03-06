// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
       
        let mut paths = vec![];
        let dfs = move |node: &Rc<RefCell<TreeNode>>, s: i32, sum: i32, path: &mut Vec<i32>| {
            s += node.borrow().val;
            path.push(node.borrow().val);
            
            if let Some(nl) = &node.borrow().left {
                dfs(nl, s, sum);
            }
            
            if let Some(nr) = &node.borrow().right {
                dfs(nr, s, sum)
            }
            
            if let None = node.borrow().left {
                if let None = node.borrow().right {
                    if s == sum {
                        paths.push(path.slice());
                    }
                }        
            }
            path.pop();
            false
        }
        
        if let Some(rn) = &root {
            dfs(rn, 0, sum, vec![]);
        }
        return paths;
    }
}

