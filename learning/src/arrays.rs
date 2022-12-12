// use std::mem;
pub fn array(){
    let array : [i32; 4] = [0,1,2,3];
    println!("{}",array[0]);
    println!("{}",array.len());
    println!("{:?}",array);
    // println!("{}",std::mem::size_of_val(&array));
    // println!("{}",mem::size_of_val(&array));
    // Get Slice of an Array
    let slice : &[i32] = &array[0..2];
    println!("{:?}",slice);
}