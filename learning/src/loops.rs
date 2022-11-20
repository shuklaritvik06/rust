pub fn loops() {
    let mut y = 0;
    for x in 1..10 {
        if x == 5 {
            continue;
        }
        println!("x is {}", x);
    }
    while y < 10 {
        y += 1;
        println!("inside loop y value is {}", y);
    }
    let mut z = 0;
    loop {
       z+=1;
       println!("z={}",z);
       if z==15 {
          break;
       }
    }
}
