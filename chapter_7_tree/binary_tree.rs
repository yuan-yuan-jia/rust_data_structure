
use std::rc::Rc;

use data_structure::include::{print_util, tree_node::TreeNode};

/*Driver code */
fn main() {
    /*初始化二叉树 */
    // 初始化节点
    let n1 = TreeNode::new(1);
    let n2 = TreeNode::new(2);
    let n3 = TreeNode::new(3);
    let n4 = TreeNode::new(4);
    let n5 = TreeNode::new(5);

    // 构建节点之间的引用(指针)
    n1.borrow_mut().left = Some(Rc::clone(&n2));
    n1.borrow_mut().right = Some(Rc::clone(&n3));
    n2.borrow_mut().left = Some(Rc::clone(&n4));
    n2.borrow_mut().right = Some(Rc::clone(&n5));

    // 打印二叉树
    println!("\n初始化二叉树\n");
    print_util::print_tree(&n1);

    //插入节点与删除节点
    let p = TreeNode::new(0);
    // 在n1 -> n2之间插入p
    p.borrow_mut().left = Some(Rc::clone(&n2));
    n1.borrow_mut().left = Some(Rc::clone(&p));
    println!("\n插入节点P后\n");
    print_util::print_tree(&n1);

    // 删除节点p
    drop(p);
    n1.borrow_mut().left = Some(Rc::clone(&n2));
    println!("\n删除节点P后\n");
    print_util::print_tree(&n1);

}