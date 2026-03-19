use dsa;

fn main() {
    let mut list = vec![5, 3, 2, 7, 1, 9, 18, 21, 4];

    println!("Unsorted vec: {:?}", list);
    dsa::merge_sort(&mut list);
    println!("Sorted vec: {:?}", list);
}
