/*

for i:=0;i<arr.len();i++{
    for j:=0;j<arr.len()-i-1;j++{
        if arr[i]>arr[j]{
            arr[i],arr[j] = arr[j],arr[i]
        }
    }
}

 */

// 冒泡排序：普通实现(范围遍历)
pub fn babble_sort_0(arr: &mut [i32; 6]) {
    for i in 0..arr.len() - 1 {
        for j in 0..arr.len() - 1 - i {
            if arr[index] < arr[j + 1] {
                // 方式 1 ：普通实现
                let tmp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;
            }
        }
    }
}

// 冒泡排序：优化
pub fn babble_sort_1(arr: &mut [i32; 6]) {
    for i in 0..arr.len() - 1 {
        // 记录最值元素索引位置
        let mut index = 0;
        for j in 0..arr.len() - 1 - i {
            if arr[index] < arr[j + 1] {
                index = j + 1;
            }
        }
        // 方式 2 : 缩减交换次数
        // 每完成一轮则判断按需至多交换一次
        if arr[index] > arr[arr.len() - 1 - i] {
            let tmp = arr[index];
            arr[index] = arr[arr.len() - 1 - i];
            arr[arr.len() - 1 - i] = tmp;
        }
    }
}