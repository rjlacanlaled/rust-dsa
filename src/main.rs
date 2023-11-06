// array
mod arrays;
mod sorting;
mod binary;
mod dynamic_programming;

fn main() {
    let mut arr = [0, 0, 0, 1, 10, 100, -2, -10, 5, 5, 10, 100];

    // insertion_sort::insertion_sort(&mut arr);
    // println!("insertion_sort {:?}", arr);

    // merge_sort::merge_sort(&mut arr);
    // println!("merge_sort {:?}", arr);

    // quick_sort::quick_sort(&mut arr);
    // println!("quick_sort {:?}", arr)

    // let min = arrays::min_rotated_sorted_array::find_min(arr.to_vec());
    // println!("min {:?}", min);

    // let target = arrays::three_sum::three_sum(arr.to_vec());
    // println!("target {:?}", target);

    // let sum = arrays::container_with_most_water::max_area(arr.to_vec());
    // println!("sum {:?}", sum);

    // let sum = arrays::three_sum_closest::three_sum_closest(arr.to_vec(), 199);
    // println!("sum {:?}", sum)

    let steps = dynamic_programming::longest_increasing_subsequence::length_of_lis(vec![1, 2, 5]);
    println!("steps {:?}", steps);
}
