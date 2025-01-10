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


/*Driver Code */
fn main() {
    /*快速排序 */
    let mut nums = [2,4,1,0,3,5];
    QuickSort::quick_sort(0, (nums.len() - 1) as i32, &mut nums);
    println!("快速排序完成后nums={:?}", nums);
}