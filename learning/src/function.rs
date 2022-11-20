pub fn func()->i32{
    // return 23;
    24
}

pub fn fun2(){
    let mut no:i32 = 5;
    mutate_no_to_zero(&mut no);
    println!("The value of no is:{}",no);
}

fn mutate_no_to_zero(param: &mut i32){
    *param = 0;
}
