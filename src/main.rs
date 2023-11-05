// array
mod arrays;
mod sorting;

fn main() {
    let mut arr = [7, 8, 1, 2, 3, 4, 5, 6];

    // insertion_sort::insertion_sort(&mut arr);
    // println!("insertion_sort {:?}", arr);

    // merge_sort::merge_sort(&mut arr);
    // println!("merge_sort {:?}", arr);

    // quick_sort::quick_sort(&mut arr);
    // println!("quick_sort {:?}", arr)

    // let min = arrays::min_rotated_sorted_array::find_min(arr.to_vec());
    // println!("min {:?}", min);

    let target = arrays::search_rotated_sorted_array::search(arr.to_vec(), 7);
    println!("target {:?}", target);
}
