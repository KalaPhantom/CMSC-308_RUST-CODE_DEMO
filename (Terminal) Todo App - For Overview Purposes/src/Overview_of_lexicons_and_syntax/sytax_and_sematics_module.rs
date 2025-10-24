
// # Datatypes in rust

// * ==============================================================<Primitves>
/// Description 
///  - The SC under this block highlights all the syntax of declaring primitive variables 
/// for rust - this is not runnable in any sense and it not called in any modules in this project
/// 
/// Unlike in any programming languages, Rust utilizes its standard library for declaring types and does not follow -
/// the same structural grammar for OOP-  unlike other languages that utilizes classes and modules to encapsulate data
/// - However, we can still replicate the same structure with the use of structs and implements


// Define Global variables from the Struct
// -- Rust Primitives
struct PrimitiveDeclaration{
    name: String,             
    initials: char,            
    age: u32,                             
    can_vote: bool,
    grades: (u32,u8,String),
    ratings: [u32;6],
    vect_: vec<u8>
}


/// # Module
/// A module is another container used to encapsulate related structs and implements
/// In C# - a module is equivalent to Namespaces
mod Module{

    // define struct here
    struct SampleStruct{
        // Your Code here
    }

    impl SampleStruct{
        // YOur methods and functions here

        fn new(){   // constructor

        }
    }
}


// -- Declare Functions and Methods here 
// Under this function, all datatypes declaration will be highlighted
/// This is an implement - a special encapsulated element used to implement a struct for defining methods, enums and functions
impl PrimitiveDeclaration{                                     

    enum sample_global_enum{                                   // sample enum declaration
        KALA,
        PHANTOM,
        LIMA,
        OSCAR,
        FOXTROT
    }
    
    fn declaring_instances(){
        let struct_primitive = PrimitiveDeclaration{           // Assigning values from the struct as an object   
            name: String::from("Kisame"),
            initials: 'S',
            age: 20,
            can_vote: true,
            grades: (12,8,"Description"),
            ratings:[45,23,67,90,100,34], 
            vect_: vec![12,4,2,5,6,3,5,3,5,3,5]
        };

        // -- Declaring Varuables

        let name = String::from("Oscar");                       // String - Unchanable
        let name2 = &mut name;                                  // String - Mutables- borrowed from name
        let num1: u32 = 12;                                     // unsigned 32 bit based integer
        let num2: i32 = num1 as i32;                            // Explecit casting from unsigned to signed integer 32 bit
        let ratings: [u32; 2] = [1,2];                          // Declaring array
        let boolean: bool = true;                               // Declaring boolean
        let character: char = 'A';                              // Character
        let person: (&str, i32, bool) = ("len len", 20, true);    // tuples
        let vects: vec![12,34,60];                              // vectors
    }

    fn do_nothing() -> () {                                      // Vector without a return value (static void)
         println!("Hello");
    }

    fn never_returns() -> ! {                                    // A function that never returns any value
        panic!("This crashes!");
    }


    // # Integer types
// Type	                Size	                        Range	                                 Example
// i8	            8-bit signed	                -128 to 127	                                let a: i8 = -5;
// u8	            8-bit unsigned	                0 to 255	                                let b: u8 = 200;
// i16,u16	        16-bit		
// i32,u32	        32-bit		
// i64,u64	        64-bit		
// i128,u128	    128-bit		
// isize,usize	    depends on CPU arch (32/64-bit)


    // # Floating point types
// | Type  | Size             | Example              |
// | ----- | ---------------- | -------------------- |
// | `f32` | 32-bit           | `let x: f32 = 3.14;` |
// | `f64` | 64-bit (default) | `let y = 2.718;`     |


// -- General
// | Category  | Type                | Example            |
// | --------- | ------------------- | ------------------ |
// | Integer   | `i32`, `u8`, etc.   | `let a: i32 = 10;` |
// | Float     | `f32`, `f64`        | `let b = 3.14;`    |
// | Boolean   | `bool`              | `let c = true;`    |
// | Character | `char`              | `let d = 'R';`     |
// | Tuple     | `("Rust", 1, true)` | `t.0`              |
// | Array     | `[1, 2, 3]`         | `a[0]`             |

// -- Tables are created using
// https://ozh.github.io/ascii-tables/

}

// * ==============================================================<Primitves>