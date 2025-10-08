fn main() {
   let numbers: [i32;5] = [1,2,3,4,5];
   println!("Numbers Array: {:?}",numbers);
   //let mix = [1,2,3,"apple",true];
   //println!("Mix Array: {:?}",mix);
   let fruits : [&str;3] = ["apple","banana","orange"];
   println!("Fruits Array: {:?}",fruits);
   println!("Fruits Array: {:?}",fruits[0]);
   println!("Fruits Array: {:?}",fruits[1]);
   println!("Fruits Array: {:?}",fruits[2]);

   //Tuples contain colelction of elemnts of fixed size 

   let human: (String,i32,bool) = ("Alice".to_string(),30 ,false);
   println!("Human Tuple : {:?}",human);
   let my_mix_tuple = ("kratos",23,true,[1,2,3,4,5]);
   println!("My Mix Tuple : {:?}",my_mix_tuple);

   //Slices are a contigious block of elemnts in a collection
   //contigious means that the elements are stored next to each other in memory
   //All the elemnts are right next to each other in memory
   let number_slice: &[i32] = &[1,2,3,4,5];
   println!("number slice : {:?}",number_slice);

   let animal_slice: &[&str] = &["lion","tiger","bear"];
   println!("animal slice : {:?}",animal_slice);

   let book_slices: &[&String] = &[&"The Great Gatsby".to_string(),&"To Kill a Mockingbird".to_string(),&"1984".to_string()];
   println!("book slice : {:?}",book_slices);
   

   //string vs string slice
   //string is a collection of characters
   //string slice is a reference to a substring
   //Memory allocation is very important in rust 
   //String is a heap allocated data structure
   //String slice is a pointer to a substring
   //Rust has very similar performance to C++
   //strings growable,mutable,owned string type
   let mut stone_cold: String = String::from("stone cold hell");
   println!("stone cold says : {}",stone_cold);
   stone_cold.push_str(" yeah");
   println!("stone cold says : {}",stone_cold);
   
   //B- &str(String slice)
   //heap vs stack
   //heap is a dynamic memory allocation
   //stack is a static memory allocation
   //heap is slower than stack
   //stack is faster than heap
   //heap is more flexible than stack
   //stack is more predictable than heap
   //heap is more prone to memory leaks
   //stack is more prone to stack overflow
   //heap is more prone to memory fragmentation
   //stack is more prone to memory leaks
   //heap is more prone to memory leaks
   //heap is more prone to memory leaks

   let string: String = String::from("Hello World");
   let slice: &str = &string;
   println!("slice : {:?}",slice);
   let slice2: &str = &string[0..5];
   println!("slice2 : {:?}",slice2);

   //C- &str(String slice)
   //heap vs stack
   //heap is a dynamic memory allocation
   //stack is a static memory allocation
   //heap is slower than stack
   //stack is faster than heap
   //heap is more flexible than stack
   //stack is more predictable than heap
   //heap is more prone to memory leaks
   //stack is more prone to stack overflow
   //heap is more prone to memory fragmentation
   //stack is more prone to memory leaks
   //heap is more prone to memory leaks
   //heap is more prone to memory leaks
}