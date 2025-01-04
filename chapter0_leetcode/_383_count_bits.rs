struct Solution; 

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut v = vec![0;(n + 1) as usize];
        for i in 1..=n {
           v[i as usize] =  v[(i & (i - 1)) as usize] + 1;
        }
        
        v
    }
}

fn main() {

}