struct Color{
    red: u8,
    green: u8,
    blue: u8
}
struct Tuples(u8,u8,u8);
struct Person{
    first_name: String,
    last_name: String
}
impl Person {
    // Construct a Person
    fn new(first: &str, last: &str)->Person{
        Person { first_name: first.to_string(), last_name: last.to_string() }
    }
    fn printname(first: &str, last: &str){
        println!("{} {}",first,last);
    }
    fn print_full(&self)->String{
        format!("{} {}",self.first_name,self.last_name)
    }
    fn set_last(&mut self,last: &str){
        self.last_name = last.to_string();
    }
}
pub fn run(){
    let a = Color{
        red: 255,
        green: 200,
        blue: 4
    };
    let b = Tuples(255,200,255);
    println!("{},{},{}",a.red,a.green,a.blue);
    println!("{}",b.0);
    let mut c = Person::new("Ritvik", "Shukla");
    c.set_last("Sharma");
    c.print_full();
    println!("{} {}",c.first_name,c.last_name);
    Person::printname("Ritvik", "Shukla");

}