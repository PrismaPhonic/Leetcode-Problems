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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        
        fn dfs(node: &Rc<RefCell<TreeNode>>, mut s: i32, sum: i32) -> bool {
            s += node.borrow().val;
            if let Some(nl) = &node.borrow().left {
                if dfs(nl, s, sum) == true {
                    return true;
                };
            }
            
            if let Some(nr) = &node.borrow().right {
                if dfs(nr, s, sum) == true {
                    return true;
                };
            }
            
            if let None = node.borrow().left {
                if let None = node.borrow().right {
                    if s == sum {
                        return true;
                    }
                }        
            }
            false
        }
        
        if let Some(rn) = &root {
            return dfs(rn, 0, sum);
        }
        return false;
    }
