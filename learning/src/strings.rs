pub fn strings(){
    let mut a = String::from("Hello World!");
    let b = a.clone();
    assert_eq!(a, b);
    println!("{}",a.len());
    a.push('R');
    println!("{}",a.len());
    a.pop();
    a.push_str("Hello");
    println!("{}",a.capacity());
    println!("{}",a.is_empty());
    println!("{}",a.contains("he"));
    println!("{}",a.starts_with("he"));
    println!("{}",a.ends_with("he"));
    println!("{}",a.as_str());
    println!("{}",a.to_string());
    println!("{}",a.to_lowercase());
    println!("{}",a.to_uppercase());
    for word in a.split_whitespace(){
        println!("{}",word);
    }
    let mut b = String::with_capacity(10);
    b.push('j');
    a = a.replace("Hello", "Hi");
    print!("{}",a);
}