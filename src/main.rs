mod insertion_sort;
mod merge_sort;
mod quick_sort;

fn main() {
    let mut arr = [
        2, 1, 1, -1, -1, -100, -1, 1, 50, 10000, 1, 233, 2321, 323, 323232, 11, 23, 3213, 5345, 4213,
        54, 5454, 545, 51231, 434213, 541, 213,
    ];
    // insertion_sort::insertion_sort(&mut arr);
    // println!("insertion_sort {:?}", arr);

    // merge_sort::merge_sort(&mut arr);
    // println!("merge_sort {:?}", arr);

    quick_sort::quick_sort(&mut arr);
    println!("quick_sort {:?}", arr)
}
