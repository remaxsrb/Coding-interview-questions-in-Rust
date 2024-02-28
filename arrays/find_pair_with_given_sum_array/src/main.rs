
// Input:   nums = [8, 7, 2, 5, 3, 1] target = 10  
// Output:   Pair found (8, 2) or Pair found (7, 3)    
// Input:   nums = [5, 2, 6, 8, 1, 9] target = 12 
// Output: Pair not found 
 
use std::collections::HashMap;

fn find_pair(nums: &Vec<i32>, target: i32) {

    //to solve this in linear time, hash map needs to be used

    let mut map = HashMap::new();
    let mut index = 0;
    for &num in nums { 

        let find = target-num; //Second pair member is found

        if let Some(&found_pair_index) = map.get(&find) {
            println!("Pair found ({}, {})", nums[found_pair_index], num);
            return;
        }

        map.insert(num, index);
        index += 1;
            
    }

    println!("Pair is not found");

}

fn main() {
   
    let mut nums = Vec::new();
    nums.push(8);
    nums.push(7);
    nums.push(2);
    nums.push(5);
    nums.push(3);
    nums.push(1);
    
    find_pair(&nums, 10)

}
