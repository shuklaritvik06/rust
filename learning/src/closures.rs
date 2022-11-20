pub fn closure(){
    let add_one = |x| x + 1;
    let five = add_one(4);
    println!("The value of five is: {}", five);
}