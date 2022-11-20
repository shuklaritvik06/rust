use std::collections::HashMap;
pub fn run() {
   let mut state_codes = HashMap::new();
   state_codes.insert("UP","Uttar Pradesh");
   state_codes.insert("DL","Delhi");

   for (key, val) in state_codes.iter() {
      println!("key: {} val: {}", key, val);
   }
   println!("{}",state_codes.contains_key(&"UP"));
}