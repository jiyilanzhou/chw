/*
0. 排序





 */
mod f0_babble_sort;
fn main() {
    // 数组声明
    let mut arr = [3, 2, 0, 1, 6, 8];
    // 冒泡排序
    f0_babble_sort::babble_sort(&mut arr);
    //println!("{:?}", arr);
    for elem in arr.into_iter().enumerate(){
        println!("{:?}",elem);
    }
    // 选择排序
}
