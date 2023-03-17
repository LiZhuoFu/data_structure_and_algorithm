use order::order::bubble_sort::{bubble_sort1, bubble_sort2,bubble_sort3,cocktail_sort};
fn main() {
    let mut nums = [54, 26, 93, 17, 77, 31, 44, 55, 20];
    //bubble_sort1(&mut nums);
    //bubble_sort2(&mut nums);
    //bubble_sort3(&mut nums);
    cocktail_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}
