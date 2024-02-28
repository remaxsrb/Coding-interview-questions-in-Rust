// Input:  { 3, 4, -7, 3, 1, 3, 1, -4, -2, -2 }  
// Output: Subarray with zero-sum exists   
// The subarrays with a sum of 0 are:   
// { 3, 4, -7 } 
// { 4, -7, 3 }
//  { -7, 3, 1, 3 } 
//  { 3, 1, -4 } 
//  { 3, 1, 3, 1, -4, -2, -2 } 
//  { 3, 4, -7, 3, 1, 3, 1, -4, -2, -2 } 

use std::collections::HashSet;


fn zero_sum_sub_array(arr: &Vec<i32>) -> bool {

   //In order to solve this problem in linear time, it is ideal to use a set

    let mut set = HashSet::new();

    let mut sum = 0;

    set.insert(0); //handle the case when zero sum array starts at index 0

    for &element in arr {

        sum+=element;

        if set.contains(&sum) {
            
            return true;
        }
        else {
            set.insert(sum);
        }

    }

    return false;


}

fn main() {
    
    let mut arr = Vec::new();

    arr.push(3);
    arr.push(4);
    arr.push(-7);
    arr.push(3);
    arr.push(1);
    arr.push(3);
    arr.push(1);
    arr.push(-4);
    arr.push(-2);
    arr.push(-2);

    let result = zero_sum_sub_array(&arr);

    if result {
        println!("Subarray with zero sum exists");
    }
    else {
        println!("Subarray with zero sum does not exist");
    }

}
