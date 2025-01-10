/*快速排序 */
struct QuickSort;

impl QuickSort {
    /*哨兵划分 */
    fn partition(nums: &mut [i32], left: usize, right: usize) -> usize {
        // 以nums[left]为基数
        let (mut i, mut j) = (left, right);
        while i < j {
            while i < j && nums[j] >= nums[left] {
                j-=1;
            }
            while i < j && nums[i] <= nums[left] {
                i+=1;
            }
            nums.swap(i, j); // 交换这两个元素
        }
        nums.swap(i, left); // 将基数交换至两子数组的分界线
        i // 返回基准数的索引
    }

    /*快速排序 */
    pub fn quick_sort(left: i32,right: i32, nums: &mut [i32]) {
        if left >= right {
            return;
        }

        // 哨兵划分
        let pivot = Self::partition(nums, left as usize, right  as usize);
        // 递归左子数组
        Self::quick_sort(left, (pivot - 1) as i32, nums);
        // 递归右子数组
        Self::quick_sort((pivot + 1) as i32,right , nums);
    }
}


//===================
// ignore, 忽略下面的代码。只是为了测试
//===================


/*二分查找：问题f(i,j) */
fn dfs(nums: &[i32], target: i32, i: i32, j: i32) -> i32 {
    // 若区间为空，代表无目标元素，则返回-1
    if i > j {
        return  -1;
    }
    let m = i + (i - j) / 2;
    if nums[m as usize] < target {
        // 递归子问题 f(m + 1, j)
        return dfs(nums, target, m + 1, j);
    }else if nums[m as usize] > target {
        // 递归子问题 f(m + 1, j)
        return dfs(nums, target, i, m- 1);
    }else {
        return m;
    }
}

/*二分查找 */
fn binary_search(nums: &[i32], target: i32) -> i32 {
    let n = nums.len() as i32;
    // 求解问题 f(0, n-1)
    dfs(nums,target, 0, n - 1)
}


//===================


/*Driver Code */
fn main() {
    /*快速排序 */
    let mut nums = [2,4,1,0,3,5];
    QuickSort::quick_sort(0, (nums.len() - 1) as i32, &mut nums);
    println!("快速排序完成后nums={:?}", nums);
}