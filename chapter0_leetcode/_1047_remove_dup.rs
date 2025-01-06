use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut v = Vec::new();
        for ele in s.chars() {
            if v.is_empty()  {
                v.push(ele);
            }else {
                if v.last().is_some() && *v.last().unwrap() == ele {
                    v.pop();
                }else {
                    v.push(ele);
                }
            }
        }

        let mut s = String::new();
        for ele in v.iter() {
            s.push(ele.clone());
        } 
        s   
    }
}

fn main() {
    let s= String::from("abbbabaaa");
    let s = Solution::remove_duplicates(s);
    println!("{}",s);
}

