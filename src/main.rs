// sort
mod insertion_sort;
mod merge_sort;
mod quick_sort;

// array
mod contains_duplicate;
mod product_except_self;
mod max_sub_array;
mod max_profit;
mod max_product_sub_array;

fn main() {
    let mut arr = [2, -5, -2, -4, 3];

    // insertion_sort::insertion_sort(&mut arr);
    // println!("insertion_sort {:?}", arr);

    // merge_sort::merge_sort(&mut arr);
    // println!("merge_sort {:?}", arr);

    // quick_sort::quick_sort(&mut arr);
    // println!("quick_sort {:?}", arr)

    max_product_sub_array::max_product(arr.to_vec());
}
