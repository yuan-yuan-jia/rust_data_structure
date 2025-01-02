
use data_structure::include::map::Pair;


/*开放寻址哈希表 */
struct HashMapOpenAddressing {
    size: usize,                    // 哈希表中键值对的数量 
    capacity: usize,                // 哈希表容量
    load_thres: f64,                // 负载因子阈值
    extend_ratio: usize,            // 扩容比例
    buckets: Vec<Option<Pair>>,// 存储键值对的桶
    TOMBSTONE: Option<Pair>,     // 墓碑,删除标记
}

impl HashMapOpenAddressing {

    fn new() -> Self {
        Self {
            size: 0,
            capacity: 4,
            load_thres: 2.0 / 3.0,
            extend_ratio: 2,
            buckets: vec![None; 4],
            TOMBSTONE: Some(Pair {
                key: -1,
                val: "-1".to_string(),
            }),
        }
    }
    
    /*哈希函数 */
    fn hash_func(&self, key: i32) -> usize {
        (key  % self.capacity as i32) as usize
    }

    /*负载因子 */
    fn load_factor(&self) -> f64 {
        self.size as f64 / self.capacity as f64
    }

    /*查询 */
    fn get(&self, key: i32) -> Option<&str> {
        let hash_ = self.hash_func(key);
        let item = self.buckets.get(hash_)?;
        let item = item.as_ref()?;
        Some(item.val.as_str())
    }

    /*删除操作 */
    fn remove(&mut self, key: i32) {
        let hash_ = self.hash_func(key);
        let entry =  self.buckets.get(hash_);
        if entry.is_some() && entry.unwrap().is_some() && entry.unwrap().as_ref() != self.TOMBSTONE.as_ref() {
            self.buckets[hash_] = self.TOMBSTONE.clone();
            self.size -= 1;
            return;
        }
    }

    /*扩容哈希表 */
    fn extend(&mut self) {
        // 保存旧的buckets
        let old_buckets = std::mem::replace(&mut self.buckets, vec![]);
        // 更新容量
        self.capacity *= self.extend_ratio;
        // 申请新的vec
        let mut new_buckets: Vec<Option<Pair>> = vec![None; self.capacity];
        let _ = std::mem::replace(&mut self.buckets, new_buckets);
        // 重新插入
        for entry in old_buckets {
            if entry.is_some() && entry.as_ref() != self.TOMBSTONE.as_ref() {
                // 重新插入
                // let hash_ = self.hash_func(entry.as_ref().unwrap().key);
                // new_buckets[hash_] = entry;
                // 调用put方法
                let pair = entry.unwrap();
                self.put(pair.key, pair.val);
            }
        }

        
    }

    /*打印哈希表 */
    fn print(&self) {
        for bucket in &self.buckets {
            if bucket.is_some() && bucket.as_ref() != self.TOMBSTONE.as_ref() {
                let pair = bucket.as_ref().unwrap();
                println!("{}->{}", pair.key, pair.val);
           }
       }
    }


    /* 搜索 key 对应的桶索引 */
    fn find_bucket(&mut self, key: i32) -> usize {
        let mut index = self.hash_func(key);
        let mut first_tombstone = -1;
        // 线性探测，当遇到空桶时跳出
        while self.buckets[index].is_some() {
            // 若遇到 key，返回对应的桶索引
            if self.buckets[index].as_ref().unwrap().key == key {
                // 若之前遇到了删除标记，则将建值对移动至该索引
                if first_tombstone != -1 {
                    self.buckets[first_tombstone as usize] = self.buckets[index].take();
                    self.buckets[index] = self.TOMBSTONE.clone();
                    return first_tombstone as usize; // 返回移动后的桶索引
                }
                return index; // 返回桶索引
            }
            // 记录遇到的首个删除标记
            if first_tombstone == -1 && self.buckets[index] == self.TOMBSTONE {
                first_tombstone = index as i32;
            }
            // 计算桶索引，越过尾部则返回头部
            index = (index + 1) % self.capacity;
        }
        // 若 key 不存在，则返回添加点的索引
        if first_tombstone == -1 {
            index
        } else {
            first_tombstone as usize
        }
    }


    /*添加操作 */
    fn put(&mut self, key: i32, val: String) {
        if self.load_factor() > self.load_thres {
            self.extend();
        }

        // 搜索key对应的桶索引
       let index = self.find_bucket(key);
       if self.buckets[index].is_some() && self.buckets[index].as_ref().unwrap().key == key {
           // 更新值， 存在相同的key
           self.buckets[index].as_mut().unwrap().val = val;
           return;
       }
       // 若键值对不存在，则添加该键值对
         self.buckets[index] = Some(Pair{key, val});
         self.size+=1;

    }
}


/*Deriver Code */
fn main() {
   /* 初始化哈希表 */
   let mut hashmap = HashMapOpenAddressing::new();

   /* 添加操作 */
   // 在哈希表中添加键值对 (key, value)
   hashmap.put(12836, "小哈".to_string());
   hashmap.put(15937, "小啰".to_string());
   hashmap.put(16750, "小算".to_string());
   hashmap.put(13276, "小法".to_string());
   hashmap.put(10583, "小鸭".to_string());

   println!("\n添加完成后，哈希表为\nKey -> Value");
   hashmap.print();

   /* 查询操作 */
   // 向哈希表中输入键 key ，得到值 val
   let name = hashmap.get(13276).unwrap();
   println!("\n输入学号 13276 ，查询到姓名 {}", name);

   /* 删除操作 */
   // 在哈希表中删除键值对 (key, val)
   hashmap.remove(16750);
   println!("\n删除 16750 后，哈希表为\nKey -> Value");
   hashmap.print();

}