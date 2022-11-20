// Pointer to a resource in memory
pub fn run(){
    let a = vec![1,2,3];
    // Moved value
    let b  = &a;
    print!("{:?}",(&a,b));
}