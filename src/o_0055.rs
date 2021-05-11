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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn update(root: Option<Rc<RefCell<TreeNode>>>,level:i32,max_level:&mut i32){
            match root{
                Some(node) =>{
                    let node = node.borrow();
                    *max_level = (*max_level).max(level+1);
                    update(node.left.clone(),level+1,max_level);
                    update(node.right.clone(),level+1,max_level);
                }
                None => {},
            }
        }
        let mut ans = 0;
        update(root,0,&mut ans);
        ans
    }
}