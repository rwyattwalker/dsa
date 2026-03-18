use dsa;

fn main() {
    let mut list = vec![5, 3, 2, 7, 1, 9, 18, 21, 4];

    println!("Unsorted vec: {:?}", list);
    insertion_sort(&mut list);
    println!("Sorted vec: {:?}", list);
}

fn insertion_sort(list: &mut Vec<u32>) {
    for j in 1..list.len() {
        let key = list[j];
        let mut i = j;
        while i > 0 && list[i - 1] > key {
            list[i] = list[i - 1];
            i -= 1;
        }
        list[i] = key;
    }
}
