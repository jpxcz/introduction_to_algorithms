use std::cmp;

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

    let a = vec![1, 0, 1, 1];
    let b = vec![0, 0, 0, 0];

    let sum = binary_add(&a, &b);
    println!("Sum: {:?}", sum);

    let c: i8 = 1 & 1;

    println!("bitwise {}", c);
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

fn binary_add(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut carry = 0;
    let n = cmp::max(a.len(), b.len());
    let mut result = Vec::with_capacity(n + 1);

    for i in 0..n {
        // get the bit if any
        let a_bit = *a.get(i).unwrap_or(&0);
        let b_bit = *b.get(i).unwrap_or(&0);

        let temp_sum = a_bit ^ b_bit ^ carry;
        carry = (a_bit & b_bit) | (a_bit & carry) | (b_bit & carry);

        result.push(temp_sum);
    }

    if carry != 0 {
        result.push(carry);
    }

    result
}
