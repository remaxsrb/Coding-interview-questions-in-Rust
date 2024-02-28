use std::collections::HashMap;

fn longest_subarray_with_equal_count_of_zeroes_and_ones(arr: &Vec<i32>) {

    let mut map = HashMap::new();

    let mut len = 0 ;
    let mut sum = 0;
    let mut ending_index = 0;

    let zero_sum_at_zero_index: i32 = -1;

    map.insert(sum, zero_sum_at_zero_index);

    for (i , &v) in arr.iter().enumerate() {

        if v == 0 {
            sum += -1;
        }
        else  {
            sum += v;
        }

        let index = i as i32;

        if let Some(&start_index) = map.get(&sum) {
            if len < index - start_index {

                len = index - start_index;
                ending_index = index;
            }
        }
        else {
            map.insert(sum, index);
        }

    }

    if ending_index > 0  {
        println!("[{}, {}]", ending_index - len +1 , ending_index);
    }
    else {
        println!("No subarray exists");
    }

}

fn main() {
    
    //let arr = vec![0, 0, 1, 0, 1, 0, 0];

    //longest_subarray_with_equal_count_of_zeroes_and_ones(&arr);

    let arr1 = vec![0, 0, 1, 1 ];

    longest_subarray_with_equal_count_of_zeroes_and_ones(&arr1);

}
