#[derive(Clone)]
/* 键值对 */
struct Pair {
    key: i32,
    val: String,
}

/*链式地址哈希表 */
struct HashMapChaining {
    size: i32,
    capacity: i32,
    load_thres: f32,
    extend_ratio: i32,
    buckets: Vec<Vec<Pair>>,
}

impl HashMapChaining {
    
    /*构造方法 */
    fn new() -> Self {
        Self {
            size: 0,
            capacity: 4,
            load_thres: 2.0 / 3.0,
            extend_ratio: 2,
            buckets: vec![Vec::new(); 4],
        }
    }

    /*哈希函数 */
    fn hash_func(&self, key: i32) -> usize {
        key as usize % self.capacity as usize
    }

    /*负载因子 */
    fn load_factor(&self) -> f32 {
        self.size as f32 / self.capacity as f32
    }

    /*删除 */
    fn remove(&mut self, key: i32) -> Option<String> {
        let hash_ = self.hash_func(key);
        let bucket =  self.buckets.get_mut(hash_);
        match bucket {
            Some(b) => {
                b.iter_mut().position(|x| x.key == key).map(|i| {
                    // 更新大小
                    self.size -= 1;
                    b.remove(i).val
                })
            },
            None => None
        }
    }


    /*扩容 */
    fn extend(&mut self) {
        // 保存旧的哈希表
        let buckets_map = std::mem::replace(&mut self.buckets, vec![]);
        
        // 初始化扩容后的新哈希表
        self.capacity *= self.extend_ratio;
        self.buckets = vec![Vec::new(); self.capacity as usize];

        // 将键值对从原哈希表搬运至新哈希表
        for bucket in buckets_map {
            for pair in bucket {
                self.put(pair.key, pair.val);
            }
        }
    }

    /*打印哈希表 */
    fn print(&self) {
        for bucket in &self.buckets {
            for pair in bucket {
                println!("{}->{}", pair.key, pair.val);
            }
        }
    }

    /*添加操作 */
    fn put(&mut self, key: i32, val: String) {
        if self.load_factor() > self.load_thres {
            self.extend();
        }
        let hash_ = self.hash_func(key);
        let bucket =  &mut self.buckets[hash_];
        
        for pair in bucket {
            if pair.key == key {
                // 更新值， 存在相同的key
                pair.val = val;
                return;
            }
        }

        // 不存在相同的key
        // 直接将新值插入末尾
        let bucket =  &mut self.buckets[hash_];
        bucket.push(Pair{key, val});
        // 更新大小
        self.size += 1;

    }

    /*查询操作 */
    fn get(&self, key: i32) -> Option<&str> {
        let hash_ = self.hash_func(key);
        let bucket =  &self.buckets[hash_];

        for pair in bucket {
            if pair.key == key {
                return Some(&pair.val);
            }
        }
        None
    }

}

fn main() { 

    /*初始化哈希表 */
    let mut map = HashMapChaining::new();

    /*添加操作 */
    map.put(12836, "小哈".to_string());
    map.put(15937, "小啰".to_string());
    map.put(16750, "小算".to_string());
    map.put(13276, "小法".to_string());
    map.put(10583, "小鸭".to_string());
    println!("\n添加完成后，哈希表为\nKey -> Value");
    map.print();

    /*查询操作 */
    // 向哈希表键入key，得到value
    println!("\n输入学号 13276,查询到姓名 {}\n",
    match map.get(13276) {
        Some(value) => value,
        None => "Not a valid Key",
    });


        /* 删除操作 */
    // 在哈希表中删除键值对 (key, value)
    map.remove(12836);
    println!("\n删除 12836 后，哈希表为\nKey -> Value");
    map.print();

}