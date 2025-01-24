
/*回溯算法：全排列 I */
fn backtrack(
    mut state: Vec<i32>,
    choices: &[i32],
    selected: &mut [bool],
    res: &mut Vec<Vec<i32>>
) {
    //  当长度等于元素数量时，记录解
    if state.len() == choices.len() {
        res.push(state);
        return;
    }

    // 遍历所有选择
    for i in 0..choices.len() {
        let choice = choices[i];
        // 剪枝：不允许重复选择元素
        if !selected[i] {
            // 尝试：做出选择，更新状态
            selected[i] = true;
            state.push(choice);
            // 进行下一轮选择
            backtrack(state.clone(), choices, selected, res);
            //回退：撤销选择，回复到之前的状态
            selected[i] = false;
            state.pop();
        }
    }
}


/*全排列 I */
fn permutation_i(nums: &mut [i32])
 -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    backtrack(
        Vec::new(),
        nums,
        &mut vec![false;nums.len()],
        &mut res
    );
    res

 }
/*Driver Code */
fn main() {
    let mut nums = [1,2,3];
    let res = permutation_i(&mut nums);

    println!("输入数组 nums = {:?}", &nums);
    println!("所有排列 res = {:?}", &res);
}