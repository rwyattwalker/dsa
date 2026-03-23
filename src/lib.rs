pub fn insertion_sort(list: &mut Vec<u32>) {
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

pub fn selection_sort(list: &mut Vec<i32>) -> () {
    for i in 0..list.len() - 1 {
        let mut smallest = i;
        for j in i + 1..list.len() {
            if list[j] < list[smallest] {
                smallest = j;
            }
        }
        list.swap(i, smallest);
    }
}

pub fn bubble_sort(list: &mut Vec<i32>) {
    for i in 0..list.len() {
        for j in (i + 1..list.len()).rev() {
            if list[j] < list[j - 1] {
                list.swap(j, j - 1);
            }
        }
    }
}

pub fn merge_sort(arr: &mut [i32]) {
    let mid = arr.len() / 2;
    if mid > 0 {
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        merge(arr, mid);
    }
}

fn merge(list: &mut [i32], mid: usize) {
    let left = list[..mid].to_vec();
    let right = list[mid..].to_vec();

    let (mut i, mut k, mut j) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            list[k] = left[i];
            i += 1;
        } else {
            list[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        list[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        list[k] = right[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted0_to_be_less_than_sorted1() {
        let mut list = vec![9, 7, 3];
        assert!(list[0] > list[1]);
        insertion_sort(&mut list);
        assert!(list[0] < list[1]);
    }
}
