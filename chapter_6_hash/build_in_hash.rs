use core::hash;
use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};
use data_structure::include::list_node::ListNode;



fn main() {
    let num = 3;
    let mut num_hasher = DefaultHasher::new();
    num.hash(&mut num_hasher);
    let hash_num = num_hasher.finish();
    println!("整数 {} 的哈希值为 {}", num, hash_num);

    
    let bol = true;
    let mut bol_hashed = DefaultHasher::new();
    bol.hash(&mut bol_hashed);
    let hash_bol = bol_hashed.finish();
    println!("布尔值 {} 的哈希值为 {}", bol, hash_bol);

    let dec: f32 = 3.14159;
    let mut dec_hashed = DefaultHasher::new();
    dec.to_bits().hash(&mut dec_hashed);
    let hash_dec = dec_hashed.finish();
    println!("浮点数 {} 的哈希值为 {}", dec, hash_dec);

    let str = "Hello 算法";
    let mut str_hasher = DefaultHasher::new();
    str.hash(&mut str_hasher);
    let hash_str = str_hasher.finish();
    println!("字符串 {} 的哈希值为 {}", str, hash_str);

    let arr = (&12836, &"小哈");
    let mut tup_hasher = DefaultHasher::new();
    arr.hash(&mut tup_hasher);
    println!("元组 {:?} 的哈希值为 {}", arr, tup_hasher.finish());

    let node = ListNode::new(3);
    let mut hasher = DefaultHasher::new();
    node.borrow().val.hash(&mut hasher);
    let hash = hasher.finish();
    println!("节点对象 {:?} 的哈希值为 {}", node, hash);

}