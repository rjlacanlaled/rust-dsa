// array
mod arrays;
mod sorting;
mod binary;

fn main() {
    let mut arr = [1, 8, 6, 2, 5, 4, 8, 3, 7];

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

    let sum = binary::sum_of_two_integers::get_sum(1, 6);
    println!("sum {:?}", sum)
}
