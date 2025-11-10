//Functions
//Entry Point of the program
//Rust expects the main function to be present in the program
//The main function is the entry point of the program
//The main function is the first function that is executed when the program is run
//The main function is the last function that is executed when the program is run
//The main function is the first function that is executed when the program is run
//Starts with the keyword fN and followign that the name of your function 
//any function or variables should be written in snake_case
//examples : fn hello_world 
//Do not use kebab case 


fn main() {

   //If and else expressions 
   //an if expression allows you to branch your code depending on a condition 
   //if the condition is true, the code inside the if block will be executed
   //if the condition is false, the code inside the else block will be executed
   //if the condition is false, the code inside the else block will be executed

   let age: u32 = 20;
   if age >= 18 {
      println!("You are an adult");
   } else {
      println!("You are not an adult");
   }
   //Soemtimes you want to check multiple conds 
   //you can use else if to check multiple conditions

   let ageTwo : u32 = 15;
   if ageTwo >= 18 {
      println!("You are an adult");
   } else if ageTwo >= 13 {
      println!("You are a teenager");
   } else {
      println!("You are a child");
   }
   //Using if in let statements 
   let is_adult: bool = if age >= 18 {true} else {false};
   println!("Is adult: {}",is_adult);
   //Loops in Rust
   //Loops are used to repeat a block of code multiple times
   //Loops are used to iterate over a collection of elements
   loop{
      println!("Hello World");
      break; //break is used to exit a loop
   }
   //One of the uses of a loop is to retry an operation that might fails 
   let mut count = 0;
   loop{
      count += 1;
      println!("Count: {}",count);
      if count == 10 {
         break;
      }
   }
   //Loop labels 
   //Loop labels are used to label a loop
   //Loop labels are used to jump to a labeled loop
   //Loop labels are used to break out of a labeled loop
   'outer: loop{
      println!("Outer loop");
      'inner: loop{
         println!("Inner loop");
         break 'outer;
      }
   }
   //While loops in rust 
   //While loops are used to repeat a block of code while a condition is true
   //While loops are used to iterate over a collection of elements
   let mut countTwo = 0;
   while countTwo < 10 {
      println!("Count: {}",countTwo);
      countTwo += 1;
   }
   //for loops in rust 
   //For loops are used to iterate over a collection of elements
   //For loops are used to iterate over a collection of elements
   for i in 0..10 {
      println!("For loop Count: {}",i);
   }
   
   //While loops can also be used to iterate over a collection of element
   //For loops in rust 
   //For loops are used to iterate over a collection of elements
   //For loops are used to iterate over a collection of elements

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

   hello_world();
   tell_height(180);
   human_id("John", 20, 180.2);

   let _X: i32 =
   {
      let price: i32 = 5;
      let quantity: i32 = 10;
      price * quantity
   };
   println!("x : {:?}",_X);

   //calling the bmi function 
   let weight_kg: f64 = 70.0;
   let height_kg: f64 = 1.75;
   let bmi: f64 = calculate_bmi(weight_kg, height_kg);
   println!("Your BMI is : {:.2}",bmi);

   new();
   newTwo();
   //print_length(&s);

   let mut x : i32 = 5;
   let r : &mut i32 = &mut x;
   *r += 1;
   *r -= 3; 
   println!("r : {:?}",r);
   println!("x : {:?}",x); 
}

struct BankAccount{
   owner: String,
   balance: f64,
   account_number: u64,
}

impl BankAccount{

   fn withdraw(&mut self, amount: f64) -> f64
   {
      println!("Withdrawing {} from account",amount);
      println!("New balance is {}",self.balance - amount);
      println!("Balance is now {}",self.balance);
      println!("The owner of the account is {}",self.owner);
      self.balance - amount
   }

   fn check_balance(&self) -> f64
   {
      println!("Checking balance");
      println!("Balance is {}",self.balance);
      return self.balance;
   }
}

fn hello_world() {
   println!("Hello World");

}

//you can insert input values 

fn tell_height(heigh:i32)
{
   println!("Your height is : {}",heigh);
}

fn human_id(name: &str, age: u32, height : f32)
{
   println!("my name is {} ,my age is {} ,my height is {} CM",name,age,height);
}

//Expressions and Statements
//Expression is anything that returns a value
//Statement is anythign that does not return a value
//Expressions are used to return values
//Statements are used to perform actions
//Statements do not return values
//Expressions are used to return values
//Expressions are used to return values

//Expression 
//-------------------
//5
//true and false
//if condtions 
//Statements
//--------------------------------
//almost all statements end with a semicolon
//var declarations are statements
//function calls are statements
//if conditions are statements
//loops are statements
//match statements are statements
//match statements are statements

//Final Example on functions 
//BMI = weight(kg)/height(m)^2

fn calculate_bmi(weight_kg: f64, height_kg: f64) -> f64
{
   return weight_kg / (height_kg * height_kg);
}

//ownership in rust 
//--------------------------------
//C,c++ need manual memory managment control issue
//garbage collector solved this issue but it is not a perfect solution. 
//but it creates slow program
//solves this issue with ownership in rust 
// principles of ownership in rust 
//1. Every value in rust has an owner.
//2. There can only be one owner at a time.
//3. When the owner goes out of scope, the value is dropped.

//examples of ownership in rust 
//--------------------------------
//1. String 
//2. Integer
//3. Float
//4. Boolean
//5. Array
//6. TupLE

//borrowing allows you to borrow references from values 
//ownership rules in rust
//1. You can only have one owner at a time.
//2. There can only be one owner at a time
//3. When the owner goes out of scope, the value is dropped.

//Example:each value in rust ahs a value tahs an owner
fn new(){
   let s = String::from("hello");
   let length = calculate_length(&s);
   println!("the lenght of the string is '{}' is {}",s,length);
}

fn calculate_length(s: &String) -> usize{
   s.len()
}


//2. there can be only one owner at a time.
fn newTwo(){
   let s = String::from("hello");
   let s2 = s.clone();
   let length = calculate_length(&s2);
   println!("the lenght of the string is '{}' is {}",s2,length);
}

//3. when the owner goes out of scope, the value is dropped.
fn newThree(){
   let s = String::from("hello");
   let length = calculate_lengthTwo(&s);
   println!("the lenght of the string is '{}' is {}",s,length);
}
fn print_length(s: &String){
   println!("the lenght of the string is '{}' is {}",s,s.len());
}

fn calculate_lengthTwo(s: &String) -> usize{
   s.len()
}
 //references and borrowing
 //safety and performance 
 //borrowing and reference are powerful features of rust 

//immutable reference
//mutable reference 
//once can be modified but not immutable reference
//mutable reference can be modified but not immutable reference
//create reference by using & symbol
// you can have either one mutable reference or multiple immutable references

 //a struct is a collection of fields
 //allows you to group related data together

 //Introduction to control FLOW IN RUST
 //iF AND else expressions 
 //an if expression allows you to branch ur code depending on a condition 
