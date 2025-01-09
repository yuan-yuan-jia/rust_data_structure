use data_structure::include::print_util;

/*大顶堆 */
struct MaxHeap {
    // 使用vector而非数组，这样无须考虑扩容问题
    max_heap: Vec<i32>,
}

impl MaxHeap {

    /*构造方法 */
    fn new(nums: &Vec<i32>) -> Self {
        // 将列表元素原封不动地添加进堆
        let mut heap = MaxHeap {
            max_heap: vec![]
        };
        // 堆化除叶节点以外的其他所有节点
        for num in nums {
            heap.push(*num);
        }
        heap
    }

    /*打印堆 */
    fn print(&self) {
        print_util::print_heap(self.max_heap.clone());
    }

    /*获取左子点的索引 */
    fn left(i: usize) -> usize {
        2 * i + 1
    }

    /*获取右子节点的索引 */
    fn right(i: usize) -> usize {
        2 * i + 2
    }

    /*获取父子节点的索引 */
    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    /*交换元素 */
    fn swap(&mut self, i: usize, j: usize) {
        self.max_heap.swap(i, j);
    }

    /*获取堆大小 */
    fn size(&self) -> usize {
        self.max_heap.len()
    }

    /*判断堆是否为空 */
    fn is_empty(&self) -> bool {
        self.max_heap.is_empty()
    }

    /*访问堆顶元素 */
    fn peek(&self) -> Option<i32> {
        self.max_heap.first().copied()
    }

    /*元素入堆 */
    fn push(&mut self, val: i32) {
        // 添加节点
        self.max_heap.push(val);
        //从底部到顶堆化
        self.sift_up(self.size() - 1);
    }

    /*从节点i开始，从底部到顶堆化 */
    fn sift_up(&mut self, mut i: usize) {
        loop {
            if i == 0 {
                // 当前节点是父节点，没必要继续进行
                break;
            }

            let parent = Self::parent(i);
            if self.max_heap[i] <= self.max_heap[parent] {
                // 当前节点比父节点还小，没必要交换
                break;
            }
            self.swap(i, parent);
            // 循环向上堆化
            i = parent;
        }
    }

    /*元素出堆 */
    fn pop(&mut self) -> i32 {
        // 判空处理
        if self.is_empty() {
            panic!("index out of bounds");
        }

        // 交换根节点与最右节点（交换首元素与尾元素）
        self.swap(0, self.size() - 1);
        //删除节点
        let val = self.max_heap.pop().unwrap();
        // 特别判断，可能当前元素只有一个
        if self.is_empty() {
            return val;
        }
        // 从顶至底堆化
        self.sift_down(0);
        // 返回堆顶元素
        val
    }

    /*从节点i开始，从顶到底堆化 */
    fn sift_down(&mut self,mut i: usize) {
        // 判断节点i,l,r中值最大的节点，记为ma
       loop {
            let (l,r, mut ma) = (Self::left(i), Self::right(i), i);
            if l < self.size() && self.max_heap[l] > self.max_heap[ma] {
                ma = l;
            }
            if r < self.size() && self.max_heap[r] > self.max_heap[ma] {
                ma = r;
            }
            if i == ma {
                // i节点最大
                break;
            }

            // 交换
            self.swap(i, ma);
            // 循环向下堆化
            i = ma;
       }
    }
}




fn main() {
    /*初始化大顶堆 */
    let mut max_heap = MaxHeap::new(&vec![9, 8, 6, 6, 7, 5, 2, 1, 4, 3, 6, 2]);
    println!("\n输入列表并建立堆后");
    max_heap.print();

    /*获取堆顶元素 */
    let peek = max_heap.peek();
    if let Some(peek) = peek {
        println!("\n堆顶元素为{}", peek);
    }

    /*元素入堆 */
    let val = 7;
    max_heap.push(val);
    println!("\n元素{}入堆后", val);
    max_heap.print();

    /*获取堆大小 */
    let size = max_heap.size();
    println!("\n堆元素数量为{}", size);

    /*判断堆是否为空 */
    let is_empty = max_heap.is_empty();
    println!("\n堆是否为空{}", is_empty);
}