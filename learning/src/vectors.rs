// Vectors are resizable arrays

use std::vec;
use std::mem;

pub fn vectors(){
    let mut array : Vec<i32> = vec![0,1,2,3];
    println!("{}",array[0]);
    println!("{}",array.len());
    println!("{:?}",array);
    array.push(23);
    // println!("{}",std::mem::size_of_val(&array));
    println!("{}",mem::size_of_val(&array));
    // Get Slice of an Array
    let slice : &[i32] = &array[0..2];
    println!("{:?}",slice);
    for x in array.iter(){
        println!("{}",x);
    }
    for y in array.iter_mut(){
        *y *= 2;
        *y += 1;
        println!("{}",y);
    }
    
}
