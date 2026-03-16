pub fn insertion_sort(list: &mut Vec<usize>) -> () {
    for j in 1..list.len() {
        let key = *&list[j];
        let mut i = j;
        while i > 0 && &list[i - 1] > &key {
            list[i] = list[i - 1];
            i = i - 1;
        }
        list[i] = key;
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
