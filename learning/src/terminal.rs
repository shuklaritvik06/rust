pub fn run() {
    let mut line = String::new();
    print!("Enter your name : ");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello , {}", line);
    println!("no of bytes read , {}", b1);
}
