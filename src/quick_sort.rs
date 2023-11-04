pub fn quick_sort(arr: &mut [i32]) {
    if arr.len() < 2 {
        return;
    }

    println!("start {:?}", arr);

    let pivot = arr.len() - 1;
    let mut i: i32 = -1;
    let mut j = 0;

    while j < pivot {
        if arr[j] < arr[pivot] {
            i += 1;
            if (i as usize) == j {
                j += 1;
                continue;
            }
            swap(arr, i as usize, j);
        }
        j += 1;

        if j == pivot {
            i += 1;
            swap(arr, i as usize, pivot);
            quick_sort(&mut arr[0..i as usize]);
            quick_sort(&mut arr[(i as usize) + 1..]);
        }
    }

    println!("end {:?}", arr);
}

fn swap(arr: &mut [i32], i: usize, j: usize) {
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
}
