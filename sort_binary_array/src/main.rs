//solution to this problem is quite straightforward, all that is needed is to count either occurrences of 0s or 1s and then populate the array accordingly

fn sort_binary_array(arr: &mut Vec<i8>) {

    let mut zero_cnt = 0;

    for i in 0..arr.len() {
        if arr[i] == 0 {
            zero_cnt+=1;
        }
    }

    for i in 0..arr.len() {
        if i <zero_cnt {
            arr[i] = 0;
        }
        else {
            arr[i] = 1;
        }
    }

}


fn main() {
    
    let mut arr: Vec<i8> = vec![1, 0, 1, 0, 1, 0, 0, 1];

    println!("{:?}", arr);

    sort_binary_array(&mut arr);

    println!("{:?}", arr);


}
