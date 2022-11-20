// fn main(){
//     println!("Hello world!");
// }

pub fn say_greeting(greeting: String){
    println!("Hello world! {}",greeting);
}

pub fn say_hello(){
    println!("Hello I am {0}, I likes to {1}, but I also love to {3} and {2}!","Ritvik","code","travel","eat");
}

pub fn say_hey(){
    println!("Hello I am {name}",name="Ritvik");
}

pub fn bin(){
    println!("Binary: {:b},Hex: {:x}, Octal: {:o}",10,10,10)
}

pub fn print_same(){
    println!("{:?}",(10,10,10))
}