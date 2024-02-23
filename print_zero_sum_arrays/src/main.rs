
use std::collections::HashMap;

fn print_all_zero_sum_subarrays (arr: &Vec<i32>) {


    let mut map = HashMap::new();

    let mut sum = 0;

    for (index, &element) in arr.iter().enumerate() {

        sum += element;

        //special case if zero sum subarray starts at index zero

        if sum == 0 {
            println!("Subarray from index 0 to {}", index);
        }

        if let Some(&prev_index) = map.get(&sum) {

            //if the current sum is already encountered it means that there is a zero sum subarray beggining at prev + 1 and ending at current index

            println!("Subarray from index {} to {}", prev_index + 1, index);

        } else {

            //if the current cumulative sum is not yet encountered, it is inserted into a map with it as the key and current index as value

            map.entry(sum).or_insert(index);

        }
    }

        

    }


fn main() {

    let mut arr = Vec::new();

    //arr.push(0);
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
    //arr.push(0);

    print_all_zero_sum_subarrays(&arr);
}
