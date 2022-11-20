pub fn conditional(){
    let _is_driver = false;
    if _is_driver {
        println!("Driver");
    }else{
        println!("Not Driver")
    }
    let result = match _is_driver {
        false => "Not a driver",
        true => "Driver",
    };
    print!("{}", result)
}