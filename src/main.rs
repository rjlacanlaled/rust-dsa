// array
mod arrays;
mod sorting;

fn main() {
    let mut arr = [4, 5, 6, 7, 0, 1, 2];

    // insertion_sort::insertion_sort(&mut arr);
    // println!("insertion_sort {:?}", arr);

    // merge_sort::merge_sort(&mut arr);
    // println!("merge_sort {:?}", arr);

    // quick_sort::quick_sort(&mut arr);
    // println!("quick_sort {:?}", arr)

    let min = arrays::min_rotated_sorted_array::find_min(arr.to_vec());
    println!("min {:?}", min);
}
