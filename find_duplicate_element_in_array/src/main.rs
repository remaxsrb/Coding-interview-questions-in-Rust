//I'm going to use a set for this problem but it can be solved in other ways such as xoring all elements within the array

use std::collections::HashSet;

fn find_duplicate_element(arr: &Vec<i32>) {

    let mut set = HashSet::new();

    for &num in arr {

        if let Some(&element) = set.get(&num) {

            println!("Duplicate element is {}", element);
            return;

        }

        set.insert(num);

    }
}

fn main() {
    
    let arr1 = vec![ 1, 2, 3, 4, 4];

    let arr2 = vec![ 1, 2, 3, 4, 2 ];

    println!("Array 1: {:?}", arr1);

    find_duplicate_element(&arr1);

    println!("Array 2: {:?}", arr2);

    find_duplicate_element(&arr2);

}
