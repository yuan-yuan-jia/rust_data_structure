struct Solution;

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let len = s.len();
        let mut index = 0;
        for word in &words {
            for ele in word.chars() {
                let ch = s.chars().nth(index);
                if ch.is_none() {
                    return false;
                }
                let ch = ch.unwrap();
                if ch != ele {
                    return false;
                }
                index+=1;
            }
            if s.chars().nth(index).is_none() {
                return true;
            }

        }

        false
    }
}

fn main() {

}