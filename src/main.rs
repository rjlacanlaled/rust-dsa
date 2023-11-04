mod insertion_sort;
mod merge_sort;

fn main() {
    let mut arr = [1, 5, 2, 4, 3, -1, 5, 1, 1, 10, 100, 1, 1, 23, 200, 1, -5, -10];
    // insertion_sort::insertion_sort(&mut arr);
    // println!("insertion_sort {:?}", arr);

    merge_sort::merge_sort(&mut arr);
    println!("merge_sort {:?}", arr);
}
