use std::{cell::RefCell, rc::Rc, sync::Arc};

use data_structure::{include::{print_util, tree_node::{vec_to_tree, TreeNode}}, op_vec};



/*前序遍历 */
fn pre_order(root: &Rc<RefCell<TreeNode>>) -> Vec<i32> {
    let mut res = vec![];

    fn dfs(node: &Rc<RefCell<TreeNode>>, res: &mut Vec<i32>) {
        res.push(node.borrow().val.clone());
        if let Some(left) =  node.borrow().left.as_ref() {
            dfs(left, res);
        }
        if let Some(right) =  node.borrow().right.as_ref() {
            dfs(right, res);
        }
    }
    dfs(&root, &mut res);
    res
}

/*中序遍历 */
fn in_order(root: &Rc<RefCell<TreeNode>>) -> Vec<i32> {
    let mut res = vec![];

    fn dfs(node: &Rc<RefCell<TreeNode>>, res: &mut Vec<i32>) {
        if let Some(left) =  node.borrow().left.as_ref() {
            dfs(left, res);
        }
        res.push(node.borrow().val.clone());
        if let Some(right) =  node.borrow().right.as_ref() {
            dfs(right, res);
        }
    }
    dfs(&root, &mut res);
    res
}

/*后序遍历 */
fn post_order(root: &Rc<RefCell<TreeNode>>) -> Vec<i32> {
    let mut res = vec![];

    fn dfs(node: &Rc<RefCell<TreeNode>>, res: &mut Vec<i32>) {
        if let Some(left) =  node.borrow().left.as_ref() {
            dfs(left, res);
        }
        if let Some(right) =  node.borrow().right.as_ref() {
            dfs(right, res);
        }
        res.push(node.borrow().val.clone());
    }
    dfs(&root, &mut res);
    res
}


/*Driver code */
fn main() {
  /* 初始化二叉树 */
    // 这里借助了一个从数组直接生成二叉树的函数
    let root = vec_to_tree(op_vec![1, 2, 3, 4, 5, 6, 7]).unwrap();
    println!("初始化二叉树\n");
    print_util::print_tree(&root);

    /* 前序遍历 */
    let vec = pre_order(&root);
    println!("\n前序遍历的节点打印序列 = {:?}", vec);

    /* 中序遍历 */
    let vec = in_order(&root);
    println!("\n中序遍历的节点打印序列 = {:?}", vec);

    /* 后序遍历 */
    let vec = post_order(&root);
    print!("\n后序遍历的节点打印序列 = {:?}", vec);
}