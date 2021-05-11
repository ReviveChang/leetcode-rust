use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

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
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(arr:&mut Vec<i32>, node: Option<Rc<RefCell<TreeNode>>>) {
            if let Some(node) = node{
                let _ref = node.borrow();
                if _ref.left.is_none() && _ref.right.is_none(){
                    arr.push(_ref.val);
                }else{
                    dfs(arr, _ref.left.clone());
                    dfs(arr, _ref.right.clone());
                }
            }
        }
        let mut tree1 = Vec::new();
        let mut tree2 = Vec::new();

        dfs(&mut tree1, root1);
        dfs(&mut tree2, root2);

        tree1 == tree2
    }
}