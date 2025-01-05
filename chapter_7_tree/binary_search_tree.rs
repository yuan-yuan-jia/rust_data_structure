use std::{borrow::Borrow, cell::RefCell, ops::Deref, rc::Rc};

use data_structure::include::{print_util, tree_node::TreeNode};



type OptionTreeNodeRc = Option<Rc<RefCell<TreeNode>>>;

/*二叉搜索树 */
pub struct BinarySearchTree {
    root: OptionTreeNodeRc,
}

impl BinarySearchTree {
    
    /*构造方法 */
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    /*获取二叉树根节点 */
    pub fn get_root(&self) -> OptionTreeNodeRc {
        self.root.clone()
    }

    /*查找节点 */
    pub fn search(&self, num: i32) -> OptionTreeNodeRc {
        let mut cur = self.root.clone();
        // 循环查找，越过叶节点后跳出
        while let Some(node) = cur.clone() {
            match num.cmp(&node.deref().borrow().val) {
                // 目标节点在cur的左子树中
                std::cmp::Ordering::Less => cur = node.deref().borrow().left.clone(),
                // 找到目标节点，跳出循环
                std::cmp::Ordering::Equal => break,
                // 目标节点在cur的右子树中
                std::cmp::Ordering::Greater => cur = node.deref().borrow().right.clone(),
            }
        }

        // 返回目标节点
        cur
    }

    pub fn insert(&mut self, num: i32) {
        if self.root.is_none() {
            self.root = Some(TreeNode::new(num));
            return;
        }

        let mut cur: OptionTreeNodeRc = self.root.clone();
        let mut pre: OptionTreeNodeRc = None;
        while let Some(node) = cur.clone() {
            match num.cmp(&node.deref().borrow().val) {
                // 要插入的节点在当前节点的左子树
                std::cmp::Ordering::Less => {
                    pre = cur.clone();
                    cur = node.deref().borrow().left.clone();
                },
                std::cmp::Ordering::Equal => {  
                    // 找到重复节点，直接重复返回
                    return;  
                    /*  nothing need to do,because the both node have same value*/
                },

                std::cmp::Ordering::Greater => {
                    // 要插入的节点在当前节点的右子树
                    pre = cur.clone();
                    cur = node.deref().borrow().right.clone();
                },
            }
        }

        let new_node = Some(TreeNode::new(num));
        let pre = pre.unwrap();
        if num > pre.deref().borrow().val {
           pre.borrow_mut().right = new_node;
        }else {
            pre.borrow_mut().left = new_node;  
        }
 
    }

    /*删除节点 */
    pub fn remove(&mut self, num: i32) {
        // 空树直接返回
        if self.root.is_none() {
            return;
        }
        let mut cur: OptionTreeNodeRc = self.root.clone();
        let mut pre: OptionTreeNodeRc = None;
        // 循环查找，越过叶节点后跳出（cur是叶节点，意味着二叉搜索树中，没有要删除的节点）
        while let Some(node) = cur.clone() {
            match num.cmp(&node.deref().borrow().val) {
                std::cmp::Ordering::Less => {
                    // 要删除的节点比当前节点的值要小
                    // 要删除的节点在当前节点的左子树中
                    pre = cur.clone();
                    cur = node.deref().borrow().left.clone();
                },
                std::cmp::Ordering::Equal => {
                    // 找到要被删除的节点
                    // 终止循环
                    break;
                },
                std::cmp::Ordering::Greater => {
                    // 要删除的节点比当前节点的值要大
                    // 要删除的节点在当前节点的右子树中
                    pre = cur.clone();
                    cur = node.deref().borrow().right.clone();
                },
            }
        }

        if cur.is_none() {
            // 没有找到要删除的节点，走到叶节点，非控制语句退出循环
            return;
        }
        let cur = cur.unwrap();
        // 根据要删除的节点的子的个数来做具体的逻辑
        let (left_child, right_child) = (cur.as_ref().borrow().left.clone(), cur.as_ref().borrow().right.clone());

        match (left_child.clone(), right_child.clone()) {
            // 子节点数量=0 or 1
            (None, None) | (None, Some(_)) | (Some(_), None) => {
                // 当子节点数量 = 0 / 1 时， child = nullptr / 该子节点
                let child = left_child.or(right_child);
                if pre.is_none() {
                    // 前一个节点为none，要删除的节点是根节点
                    self.root = child;
                }else {
                    // 要删除的节点不是根节点
                    // 安全地unwrap
                    let pre = pre.unwrap();
                    let left = pre.deref().borrow().left.clone();
                    if left.is_some() && Rc::ptr_eq(left.as_ref().unwrap(), &cur) {
                        pre.deref().borrow_mut().left = child;
                    }else {
                        pre.deref().borrow_mut().right = child;
                    }
                }
            
            },
            // 子节点的数量为2
            (Some(_), Some(_)) => {
                // 获取中序遍历中cur的下一个节点
                let mut tmp = cur.deref().borrow().right.clone();
                while let Some(node) = tmp.clone() {
                    if node.deref().borrow().left.is_some() {
                        tmp = node.deref().borrow().left.clone();
                    }else {
                        break;
                    }

                    let tmp_val = tmp.clone().unwrap().deref().borrow().val;
                    // 递归删除节点tmp_val
                    self.remove(tmp_val);
                    // 用tmp_value直接覆盖, 不破坏原有结构
                    cur.as_ref().borrow_mut().val = tmp_val;
                }
            },
        }

    }

}



/*Driver Code */
fn main() {
   /*初始化二叉搜索树 */
   let mut bst = BinarySearchTree::new();
    // 请注意，不同的插入顺序会生成不同的二叉树，该序列可以生成一个完美二叉树
    let nums = [8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15];

    for num in &nums {
        bst.insert(num.clone());
    }
    println!("\n初始化的二叉树\n");
    print_util::print_tree(bst.get_root().as_ref().unwrap());

    /*查找节点 */
    let node = bst.search(7);
    println!("\n查找到的节点对象为{:?},节点值 = {}", 
            node.clone().unwrap().deref().borrow(), 
            node.clone().unwrap().deref().borrow().val
        );

    /*插入节点 */
    bst.insert(16);
    println!("\n插入节点16后,二叉树为");
    print_util::print_tree(bst.get_root().as_ref().unwrap());

    /*删除节点 */
    bst.remove(1);
    println!("\n删除节点1后，二叉树为\n");
    print_util::print_tree(bst.get_root().as_ref().unwrap());
    bst.remove(2);
    println!("\n删除节点2后，二叉树为\n");
    print_util::print_tree(bst.get_root().as_ref().unwrap());
    bst.remove(4);
    println!("\n删除节点4后，二叉树为\n");
    print_util::print_tree(bst.get_root().as_ref().unwrap());
    
}