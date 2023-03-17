pub fn bubble_sort1(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }
    for i in 1..nums.len() {
        for j in 0..nums.len() - i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}

pub fn bubble_sort2(nums: &mut [i32]) {
    let mut len = nums.len() - 1;
    while len > 0 {
        for i in 0..len {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
            }
        }
        len -= 1;
    }
}
pub fn bubble_sort3(nums: &mut [i32]) {
    let mut compare = true;
    let mut len = nums.len() - 1;

    while len > 0 && compare {
        compare = false;
        for i in 0..len {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                compare = true; // 数据无序 ， 还 需 继 续 比 较
            }
        }
        len -= 1;
    }
}
pub fn cocktail_sort(nums: &mut [i32]) {
     if nums.len() <= 1 { return; }
    
    // bubble 控 制 是 否 继 续 冒 泡
     let mut bubble = true;
     let len = nums.len();
     for i in 0..(len >> 1) {
     if bubble {
     bubble = false;
    
     // 从 左 到 右 冒 泡
     for j in i..(len - i - 1) {
     if nums[j] > nums[j+1] {
     nums.swap(j, j+1);
     bubble = true
    
     }
     }
    
     // 从 右 到 左 冒 泡
     for j in (i+1..=(len - i - 1)).rev() {
     if nums[j] < nums[j-1] {
     nums.swap(j-1, j);
     bubble = true
     }
     }
     } else {
     break;
     }
     }
     }
