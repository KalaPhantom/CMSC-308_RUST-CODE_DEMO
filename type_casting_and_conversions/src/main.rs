
//*  Note
/// ! Rust only contains Explicit type conversions
/// 
/// src = https://www.reddit.com/r/rust/comments/jqm0rv/rust_type_checking/
/// src = https://rustc-dev-guide.rust-lang.org/method-lookup.html
/// src = https://doc.rust-lang.org/rust-by-example/custom_types.html

//# ========================================================<using implicit conversions>

//Import
use std::fmt::Alignment;                             
use std::any::type_name;

struct Conversions{
    name: String
}

impl Conversions{

    // Type checker that returns string from Type <T> interface
    fn type_of<T>(&self,_: &T) -> &'static str {             // Type checker
        type_name::<T>()                                     // return type
    }
   
    // using 'as' keyword for conversion
    fn conversions_from_ints_1(&self){
        let n1: i32 = 10;                   // Base number 
        let n2: f64 = n1 as f64;            // convert integer to float
        let n3: u8 = n1 as u8;              // convert i32 to u8
        let n4: u128 = n2 as u128;          // converys f64 to u128 (unsigned)
        

        println!("\n\nn1 = {}, n2 = {}, n3 = {}, n4 = {}", n1,n2,n3,n4);           // display methpd
        print!("type of n1: {} - n2 {}  ,  n3 - {}, n4 - {}", 
        self.type_of(&n1), 
        self.type_of(&n2),
        self.type_of(&n3),
        self.type_of(&n4));
   }

   // using into() for safe conversions
   fn conversions_from_ints_2(&self){

        // Only for safe conversions
        let n1: i32 = 10;                   // Base number 
        let n2: f64 = n1.into();            // convert integer to float
        let n3: i8 = n1 as i8;              // using as to convert to safe int type 
        let n4: i32 = n3.into();            // converting i8 to i32 (safe)

    
        print!("\nConversion using \"into\"");
        println!("\nn1 = {}, n2 = {}, n3 = {}, n4 = {}", n1,n2,n3,n4);           // display methpd
        print!("type of n1: {} - n2 {}  ,  n3 - {}, n4 - {}\n", 
        self.type_of(&n1), 
        self.type_of(&n2),
        self.type_of(&n3), 
        self.type_of(&n4));
   }

   // string conversions
   fn string_conversion(&self){           

        // int to string
        let num = 42;
        let s = num.to_string();
        println!("{}", s); // "42"

        // string to int
        let s = "42";
        let num: i32 = s.parse().unwrap();
        println!("{}", num); // 42



   }

   // --TODO - Add more errors here
   fn invalid_ (&self){         //! Invalid conversion | will produce an error
        let a: i32 = 10;
        let b: f64 = 2.5;

        // Invalid conversion (Rust does not auto-convert types)
        // let c = a + b;      // -- Uncomment this to see the error cast
   }

   // Struct constructor
   fn new(name:String) -> Self{
     Self {name}
   }

}

//# ========================================================<using implicit conversions>



// Main method
fn main() {
   
   let mut conversion_functions = Conversions::new(String::from(""));
   conversion_functions.conversions_from_ints_1();
   conversion_functions.conversions_from_ints_2();
   conversion_functions.string_conversion();
 
   
}
