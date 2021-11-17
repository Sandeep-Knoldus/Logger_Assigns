use log::*;
fn binary_search(item: i32, arr: [i32; 6]) -> i32 {
    env_logger::init();
    info!("Binary Search");
    let mut idx_pos = -1;

    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;

        warn!("{:?}", "`if` chain at 14 can be rewritten with `match`");
        if arr[mid] > item {
            right = mid - 1;
        } else if arr[mid] < item {
            left = mid + 1;
        } else {
            left = mid;
            break;
        }
    }

    if arr[left] == item {
        idx_pos = left as i32;
        return idx_pos;
    }
    idx_pos
}
fn main() {
    let item = -17;
    let arr: [i32; 6] = [77, 25, -17, 1, 48, 7];
    let index = binary_search(item, arr);

    if index >= 0 {
        debug!("Element found at position: {}", index);
    } else {
        error!("Index negative!!!");
        debug!("Element not found");
    }
}
