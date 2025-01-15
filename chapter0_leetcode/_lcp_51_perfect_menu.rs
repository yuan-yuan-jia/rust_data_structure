struct Solution;
impl Solution {


    fn compute_max_tasty(
        materials: &mut Vec<i32>, 
        cookbooks: &Vec<Vec<i32>>, 
        attribute: &Vec<Vec<i32>>, 
        current_limit: &mut i32,
        current_tasty: &mut i32,
        current_book_index: usize,
        books_len: usize,
    ) {
        if current_book_index >= books_len {
            return;
        }

        // 检查材料能否为当前菜谱
        let cookbook = cookbooks.get(current_book_index).unwrap();
        let mut has_enough_material = true;
        for i in 0..materials.len() {
            let mut mm = materials.get_mut(i).unwrap();
            let cc = cookbook.get(i).unwrap();
            if cc > mm {
                has_enough_material = false;
                break;
            }
        }

        if !has_enough_material {

            // 直接下一本书
            Self::compute_max_tasty(
                materials, 
                cookbooks, 
                attribute, 
                current_limit,
                current_tasty,
                current_book_index + 1,
                books_len,
            );
            return;
        }



    // 收集材料
    for i in 0..materials.len() {
        let mut mm = materials.get_mut(i).unwrap();
        let cc = cookbook.get(i).unwrap();
        *mm = *mm - *cc;
    }

    // 增加饱腹感和美味度
    let aa = attribute.get(current_book_index).unwrap();
    *current_limit = *current_limit + aa.get(1).unwrap();
    *current_tasty = *current_tasty + aa.get(0).unwrap();



    // 下一本书
    Self::compute_max_tasty(
        materials, 
        cookbooks, 
        attribute, 
        current_limit,
        current_tasty,
        current_book_index + 1,
        books_len,
    );


    }


    pub fn perfect_menu(materials: Vec<i32>, cookbooks: Vec<Vec<i32>>, attribute: Vec<Vec<i32>>, limit: i32) -> i32 {
        let mut max_tasty = -1;
        let num_of_cookbooks = cookbooks.len();
        for i in 0..num_of_cookbooks {
            let mut copied_materials = materials.clone();
            let mut current_limit = 0;
            let mut current_tasty = 0;
            Self::compute_max_tasty(
                &mut copied_materials,
                &cookbooks,
                &attribute,
                &mut current_limit,
                &mut current_tasty,
                i,
                num_of_cookbooks
            );

            if current_limit >= limit && current_tasty > max_tasty {
                max_tasty = current_tasty;
            }
        }

        max_tasty

    }
}
