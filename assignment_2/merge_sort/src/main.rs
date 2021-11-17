use log::*;
fn merge(mut arr: Vec<i32>, left: usize, mid: usize, right: usize) -> Vec<i32> {
    info!("In 'merge' function");
    let array1 = mid - left;
    let array2 = right - mid;
    let new_array1 = arr.clone();
    let new_array2 = arr.clone();
    let final_array1 = &new_array1[left..mid];
    let final_array2 = &new_array2[mid..right];

    let mut loop1 = 0;
    let mut loop2 = 0;
    let mut arr_body = left;
    while loop1 < array1 && loop2 < array2 {
        if final_array1[loop1] < final_array2[loop2] {
            arr[arr_body] = final_array1[loop1];
            loop1 += 1;
        } else {
            arr[arr_body] = final_array2[loop2];
            loop2 += 1;
        }
        arr_body += 1;
    }
    while loop1 < array1 {
        arr[arr_body] = final_array1[loop1];
        loop1 += 1;
        arr_body += 1;
    }

    while loop2 < array2 {
        arr[arr_body] = final_array2[loop2];
        loop2 += 1;
        arr_body += 1;
    }
    arr
}

fn merge_sort(mut arr: Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    info!("In 'merge_sort' function");
    if right - 1 > left {
        let mid = left + (right - left) / 2;
        arr = merge_sort(arr, left, mid);
        arr = merge_sort(arr, mid, right);
        arr = merge(arr, left, mid, right);
    }
    arr
}

fn main() {
    env_logger::init();
    info!("Merge Sort");
    let mut arr: Vec<i32> = vec![94, 1, 119, -87, 50, 43, 48];
    arr = merge_sort(arr.clone(), 0, arr.len());
    debug!("Sorted array is {:?}", arr);
}
