#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

fn insertion_sort<T: Ord>(array: &mut [T]) {
    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && array[j - 1] > array[j] {
            array.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[test]
pub fn insertion_sort_1() {
    let mut v = vec!["p", "e", "t", "e", "r"];
    insertion_sort(&mut v);
    assert_eq!(v, ["e", "e", "p", "r", "t"]);
}
