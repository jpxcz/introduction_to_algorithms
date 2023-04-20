fn main() {
    let mut arr = [5, 3, 8, 1, 4, 2];
    sort_algorithm(&mut arr);
    println!("Sort {:?}", arr);

    let find: i32 = 7;
    let found = search_algorithm(&find, &arr);
    match found {
        Some(idx) => println!("Index found {}", idx),
        None => println!("No index found"),
    }
}

fn sort_algorithm<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        // to change from incremental to decremental just change the second
        // conditional > or < sign
        while j > 0 && arr[j] > arr[j - 1] {
            arr.swap(j, j - 1);
            j = j - 1;
        }
    }
}

fn search_algorithm<T: PartialEq>(search: &T, arr: &[T]) -> Option<usize> {
    for (index, item) in arr.iter().enumerate() {
        if item == search {
            return Some(index);
        }
    }
    None
}
