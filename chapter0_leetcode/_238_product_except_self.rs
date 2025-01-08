struct Solution; 

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        let mut nums = nums;
        // 求前缀
        let mut prefix = 1;
        for i in &nums {
            result.push(prefix);
            prefix = prefix * (i);
        }
        // 求后缀
       
        let mut index = (nums.len() - 1) as i32;
        let mut subfix = 1;
        while index >= 0 {
            result[index as usize] = result[index as usize] * subfix;
            subfix = subfix * nums[index as usize];
            index-=1;
        }
        result

    }
}

fn main() {
    
}