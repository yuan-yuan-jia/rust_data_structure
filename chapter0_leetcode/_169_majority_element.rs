struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut vote = 0;
        let mut majority_ele = 0;
        for i in &nums {
            if vote == 0 {
                vote = 1;
                majority_ele = *i;
            }else {
                if *i == majority_ele {
                    vote+=1;
                }else {
                    vote -= 1;
                }
            }
        }
        majority_ele

    }
}

fn main() {

}