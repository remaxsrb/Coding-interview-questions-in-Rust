use std::i32;

fn swap(arr: &mut Vec<i32>, x: usize, y: usize) {
    let temp = arr[x];
    arr[x] = arr[y];
    arr[y] = temp;
}

// one other linear solution is possible and that is simple counting sort but that requires two array traversals instead of one


fn sort_array_of_zeros_ones_and_twos(arr : &mut Vec<i32>) {

    let pivot = 1;
    let mut start = 0;
    let mut mid = 0;
    let mut end = arr.len() - 1;

    while mid <= end {

        if arr[mid] < pivot {

            swap(arr, start, mid);
            start += 1;
            mid += 1;

        }
        else if arr[mid] > pivot {

            swap(arr, mid, end);
            end -= 1;


        }
        else {
            mid += 1;
        }
    }

}

fn main() {
    
    let mut arr = vec![ 0, 1, 2, 2, 1, 0, 0, 2, 0, 1, 1, 0];

    for &e in arr.iter() {
        print!("{}", e);
    }
    println!();
    sort_array_of_zeros_ones_and_twos(&mut arr);

    for &e in arr.iter() {
        print!("{}", e);
    }

}
