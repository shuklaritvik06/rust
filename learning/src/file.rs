use std::{io::Read,io::Write};
// pub fn run() {
//     let file = File::open("text.txt");
//     match file {
//         Ok(file) => {
//             println!("file found {:?}", file);
//         }
//         Err(e) => {
//             println!("file not found \n{:?}", e);
//         }
//     }
//     println!("End of main!");
// }

pub fn read_file() {
    let mut file = std::fs::File::open("/home/ritvik/rust/learning/src/temp.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

pub fn write_file() {
    println!("{}", std::env::current_dir().unwrap().display());
    let mut file = std::fs::File::create("/home/ritvik/rust/learning/src/temp.txt").unwrap();
    file.write_all("Hello World".as_bytes())
        .expect("write failed");
}

pub fn delete_file() {
    std::fs::remove_file("/home/ritvik/rust/learning/src/temp.txt").expect("could not remove file");
}
