
//*  Note
/// ! Rust only contains Explicit type conversions
/// 
/// Sources for additional point of references
/// src = https://www.reddit.com/r/rust/comments/jqm0rv/rust_type_checking/
/// src = https://rustc-dev-guide.rust-lang.org/method-lookup.html
/// src = https://doc.rust-lang.org/rust-by-example/custom_types.html
/// src = https://users.rust-lang.org/t/how-check-type-of-variable/33845 - type checking
/// src = https://rustc-dev-guide.rust-lang.org/type-inference.html - type inferences
/// src = https://doc.rust-lang.org/std/index.html#containers-and-collections - containers anc collections
/////-- TODO: Finalize all SC blocks 
///// -- Add more informatcis like tables
/// 


//# ========================================================<using implicit conversions>

//Import
use std::fmt::Alignment;                             
use std::any::type_name;


struct Conversions{
    name: String
}

impl Conversions{

    

    // # Type Checking
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
        
        println!("=========================================================");
        println!("\n\nResult of Conversion using \"as\"" );
        println!("\nn1 = {}, n2 = {}, n3 = {}, n4 = {}", n1,n2,n3,n4);           // display methpd
        println!("type of n1: {} - n2 {}  ,  n3 - {}, n4 - {}", 
        self.type_of(&n1), 
        self.type_of(&n2),
        self.type_of(&n3),
        self.type_of(&n4));
         println!("\n=========================================================\n");
   }

   // using into() for safe conversions
   fn conversions_from_ints_2(&self){

        // Only for safe conversions
        // Using into() function for safe conversions
        let n1: i32 = 10;                   // Base number 
        let n2: f64 = n1.into();            // convert integer to float
        let n3: i8 = n1 as i8;              // using as to convert to safe int type 
        let n4: i32 = n3.into();            // converting i8 to i32 (safe)

        println!("\n=========================================================\n");
        println!("\nConversion using \"into\"");
        println!("\nn1 = {}, n2 = {}, n3 = {}, n4 = {}", n1,n2,n3,n4);           // display methpd
        print!("type of n1: {} - n2 {}  ,  n3 - {}, n4 - {}\n", 
        self.type_of(&n1), 
        self.type_of(&n2),
        self.type_of(&n3), 
        self.type_of(&n4));
        println!("\n=========================================================\n");
   }

      // string conversions
   fn string_conversion(&self){           

     
        println!("\n=========================================================\n");
        println!("=== String and Integer Conversions ===");

        // int to string
        let num = 42;
        let s = num.to_string();
        println!("{} is a type of {}", s, self.type_of(&s)); // "42"

        // string to int
        let s = "42";
        let num: i32 = s.parse().unwrap();
        println!("{} is a type of {}", num, self.type_of(&num)); // 42



   }

   // Rust supports type inferences similar to java script
   fn type_inferences(&self){
        let a = 12;                           // compiler assigns this as a i32
        let b = "Sample String";             // Treated as a string
        let mut c = 12.34;                    // Treated as a float (base 64 datatype)

        
        println!("\n=========================================================\n");
        println!("\n == Type Inferences Result ");                                         // display methpd
        print!("type of a: {} - \nb:  {} \nc: - {}\n", 
        self.type_of(&a), 
        self.type_of(&b),
        self.type_of(&c));

        println!("\n=========================================================\n");
   }

   // ! Error Demonstrations - on types
   /// Call this function for typechecking
   fn invalid_ (&self){         //! Invalid conversion | will produce an error
        let a: i32 = 10;
        let b: f64 = 2.5;

        // Invalid conversion (Rust does not auto-convert types)
        //let c = a + b;      // -- Uncomment this to see the error cast

        // Integer Overflow
        let big: u16 = 1000;                           // u16 declaration
        let small: u8 = big as u8;                     // This is allowed but the data will be lost (u16 - converted to u8)
        println!("{}", small);                         // Outputs 232 - the maximum lenght of u8


        //-- String conversion on to int types - (Accepted on JavaScript) - Uncomment to see the error
        // let s = "hello";
        // let num: i32 = s.parse().unwrap(); 

        //-- Conversion of structs - This structure is accepted in JS but rejected in rust
        struct Celsius(f64);
        struct Fahrenheit(f64);

        // let c = Celsius(30.0);           // -- Uncomment to see the error
        // let f: Fahrenheit = c;          //! --> I nvalid Conversion 

   }

   // Struct constructor
   fn new(name:String) -> Self{
     Self {name}
   }
 
}

//# ========================================================<using implicit conversions>

// Main method
fn main() {
   
   // Declare an objet using the struct
   // All functions with '&self' parameter will be embedded in the struct - can be called under the scope 
   // - where the struct was instanciated
   let mut conversion_functions = Conversions::new(String::from(""));  

   // Function calls
   conversion_functions.conversions_from_ints_1();
   conversion_functions.conversions_from_ints_2();
   conversion_functions.string_conversion();
   conversion_functions.type_inferences();


   // type cheking examples and exceptions --> Uncomment to see the results

   // ! Errors and Examples - Uncomment hte source code below to demonstrate errors
   // conversion_functions.invalid_();
}
