use std::{i32::MAX, i32::MIN};

fn find_maximum_product_of_two_elements_in_array (arr: &Vec<i32>) {


    let mut max1 = arr[0];
    let mut max2 = MIN;

    let mut min1 = arr[0];
    let mut min2 = MAX;

    for &e  in arr.iter().skip(1) {
        
        if e > max1 {
            max2 = max1;
            max1 = e;
        }

        else if e > max2 { 
            max2 = e;
        }

        if e < min1 {

            min2 = min1;
            min1 = e;
            
        }
        else if e < min2 { 
            min2 = e;
        }
    }

    if max1 * max2 > min1 * min2 {
        println!("Integers that give the maximum product are {} and {}", max1, max2);
    }
    else if max1 * max2 < min1 * min2 {
        println!("Integers that give the maximum product are {} and {}", min1, min2);
    }
    else if max1 * max2 == min1 * min2 {
        println!("Integer pairs that give the maximum product are [{}, {}] and [{}, {}]", min1, min2, max1, max2);
    }

}

fn main() {
    

    let arr = vec![ -10, -3, 5, 6, -2 ];

    find_maximum_product_of_two_elements_in_array(&arr);

}
