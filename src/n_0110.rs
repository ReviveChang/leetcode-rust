pub struct Solution;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn trav(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32){
            if let Some(node) = root {
                let node = node.borrow();
                let (left_bool,left_depth) = trav(node.left.clone());
                let (right_bool,right_depth) = trav(node.right.clone());
                if left_bool && right_bool && (left_depth-right_depth).abs() <= 1 {
                    (true,left_depth.max(right_depth)+1)
                }else{
                    (false,0)
                }
            }
            else {
                (true,0)
            }
        }
        
        let (res,_) = trav(root);
        res
    }
}