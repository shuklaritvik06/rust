mod basics;
mod types;
mod strings;
mod tuples;
mod  arrays;
mod  vectors;
mod conditionals;
mod loops;
mod function;
mod closures;
mod structs;
mod pointers;
mod enums;
mod cli;
mod hashmap;
mod file;
mod generics;
mod terminal;
fn main() {
    basics::say_greeting("jest".to_string());
    basics::say_hello();
    basics::say_hey();
    basics::bin();
    basics::print_same();
    calc();
    constant();
    types::hello();
    let _is_driver:bool = true;
    strings::strings();
    tuples::tup();
    arrays::array();
    vectors::vectors();
    conditionals::conditional();
    loops::loops();
    function::func();
    function::fun2();
    closures::closure();
    structs::run();
    pointers::run();
    cli::run();
    hashmap::run();
    // file::run();
    generics::run();
    terminal::run();
    file::write_file();
    file::read_file();
    file::delete_file();
}

fn calc(){
    println!("Calculating {}",10+10)
}

fn constant(){
    const ID: i32  = 001;
    println!("{ID}");
}