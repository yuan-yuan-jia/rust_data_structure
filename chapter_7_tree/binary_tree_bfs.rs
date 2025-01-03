use data_structure::include::tree_node::vec_to_tree;
use data_structure::include::{print_util, tree_node::TreeNode};
use data_structure::op_vec;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/*层序遍历 */
fn level_order(root: &Rc<RefCell<TreeNode>>) -> Vec<i32> {
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());

    // 初始化一个列表，用于保存层序遍历的结果
    let mut vec = Vec::new();

    while let Some(node) = queue.pop_front() {
        vec.push(node.borrow().val.clone());

        if let Some(left) = node.borrow().left.as_ref() {
            queue.push_back(left.clone()); // 将左子树节点加入队列
        }

        if let Some(right) = node.borrow().right.as_ref() {
            queue.push_back(right.clone()); // 将右子树节点加入队列
        }
    }

    vec

}


/*Driver Code */
fn main() {
    /*初始化二叉树 */
    let root = vec_to_tree(op_vec!(1, 2, 3, 4, 5, 6, 7)).unwrap();
    println!("\n初始化二叉树\n");
    print_util::print_tree(&root);

    /*层序遍历 */
    let vec = level_order(&root);
    println!("\n层序遍历的节点打印序列 = {:?}", vec);
}