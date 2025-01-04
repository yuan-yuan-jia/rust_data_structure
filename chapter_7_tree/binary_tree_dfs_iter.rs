use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use data_structure::{include::{print_util, tree_node::{vec_to_tree, TreeNode}}, op_vec};


/*前序遍历 */
fn pre_order(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
   let mut v = Vec::new();
   let mut stack = VecDeque::new();
   if root.is_none() {
     return  v;
   }
   stack.push_back(root.as_ref().unwrap().clone());
 

   while !stack.is_empty()  {
     let node = stack.pop_back();

     if let Some(n) = node {
        v.push(n.borrow().val.clone());
        if let Some(right ) = n.borrow().right.clone() {
            stack.push_back(right);
        }
        if let Some(left ) = n.borrow().left.clone() {
            stack.push_back(left);
        }
        
     }

   }

   v
}


fn main() {
 /* 初始化二叉树 */
    // 这里借助了一个从数组直接生成二叉树的函数
    let root = vec_to_tree(op_vec![1, 2, 3, 4, 5, 6, 7]).unwrap();
    println!("初始化二叉树\n");
    print_util::print_tree(&root);

    /* 前序遍历 */
    let vec = pre_order(&Some(root));
    println!("\n前序遍历的节点打印序列 = {:?}", vec);

}