use std::collections::HashMap;

//idea is to store


fn maximum_length_subarray(arr: &Vec<i32>, target: i32) {

    let mut map = HashMap::new();

    let mut sum = 0;

    let mut len = 0;

    let mut ending_index = -1;

    map.insert(sum, ending_index);

    for (i, &num) in arr.iter().enumerate() {

        sum += num;

        let index = i as i32;

        if None == map.get(&sum) {
            map.insert(sum, index);
        }

        if let Some(&found_start_index) = map.get(&(sum - target)) {

            if len < index - found_start_index {

                len = index - found_start_index;
                ending_index = index;

            }
            
        }

    }

    println!("[ {}, {} ]", ending_index - len +1, ending_index);




}

fn main() {
    
    let arr = vec![ 5, 6, -5, 5, 3, 5, 3, -2, 0 ];

    let target = 8;

    maximum_length_subarray(&arr, target);

}
