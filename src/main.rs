// sort
mod insertion_sort;
mod merge_sort;
mod quick_sort;

// array
mod contains_duplicate;
mod product_except_self;

fn main() {
    let mut arr = [2, 3, 4, 5];
    // insertion_sort::insertion_sort(&mut arr);
    // println!("insertion_sort {:?}", arr);

    // merge_sort::merge_sort(&mut arr);
    // println!("merge_sort {:?}", arr);

    // quick_sort::quick_sort(&mut arr);
    // println!("quick_sort {:?}", arr)

    product_except_self::product_except_self(arr.to_vec());
}
