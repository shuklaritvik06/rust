struct Data<T> {
    value: T,
}
pub fn run() {
    let t: Data<i32> = Data { value: 350 };
    println!("value is :{} ", t.value);
    let t2: Data<String> = Data {
        value: "Ritvik".to_string(),
    };
    println!("value is :{} ", t2.value);
}
