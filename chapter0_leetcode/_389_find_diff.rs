
struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut res = 0;
        for c in s.chars() {
            res ^= c as u8;
        }
        for c in t.chars() {
            res ^= c as u8;
        }
        res as char
    }
}

fn main() {

}