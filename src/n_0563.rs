pub struct Solution;

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


use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn traverse(root:Option<Rc<RefCell<TreeNode>>>, tilt:&mut i32) -> i32 {
            if let Some(root) = root {
                let root = root.borrow();
                let left = traverse(root.left.clone(),tilt);
                let right = traverse(root.right.clone(),tilt);

                *tilt += (left-right).abs();
            
                left+right+root.val
            }
            else {0}
        }
        
        let mut tilt = 0;
        traverse(root,&mut tilt);

        tilt
    }
}