//array x is m in length and y is n in length; m>n
fn swap(x: &mut i32, y : &mut i32)  {
    let temp = *x;
    *x = *y;
    *y = temp;

}


fn in_place_merge(arr_x: &mut Vec<i32>, arr_y: &mut Vec<i32>) {

    for e_x in arr_x.iter_mut() {

        let mut first = &mut arr_y[0];

        if *e_x > *first {
            //if true, swap and then rearange elements in y
            swap(e_x, &mut first);
            arr_y.sort();
            
        }

    }

}

fn main() {
    
    let mut x = vec![1, 4, 7, 8];
    let mut y = vec![0, 2, 3, 5, 6, 9];

    for &e in x.iter() { 
        print!("{} ", e);
    }
    println!();
    for &e in y.iter() { 
        print!("{} ", e);
    }

    println!();

    in_place_merge(&mut x, &mut y);

    for &e in x.iter() { 
        print!("{} ", e);
    }
    println!();
    for &e in y.iter() { 
        print!("{} ", e);
    }

}
