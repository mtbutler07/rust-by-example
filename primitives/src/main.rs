fn main() {
    
    /*
    * Primitive Scalar Types (Holds one value at a time)
    * - Signed Integers: i8, i16, i32, i64, i128, and isize     [Can hold +- integers]
    * - Unsigned Integers: u8, u16, u32, u64, u128, and usize   [Can hold + integers]
    * - Floating Point: f32, f64
    * - Character: char                                         [Can hold Unicode values]
    * - Boolean: bool (true or false)
    * - Null: ()
    *
    *
    * Compound Types (Can hold multiple values)
    * - Arrays: [1, 2, 3]
    * - Tuples: (1, true)
    */

    /*
    * Variables can always be type annotated. Numbers may additionally be annotated via a suffix
    * or by default. Integers default to i32 and floats to f32 without annotations. Rust can 
    * infer types from context.
    */

    // Type annotations
    let _logical: bool = true;
    let _a_float: f64 = 1.0;     // Regular annotation 
    let _a_integer = 5i32;       // Suffix annotation - don't like....

    let _default_float = 3.0;    // Default float annotation - f64
    let _default_int = 3;        // Default integer annotation = i32

    // Types can be inferred from context
    let mut _inferred_type = 12; // Type i64 is inferred
    _inferred_type = 4294967296i64;

    // Mutable variable's value can be changed
    let mut mutable_variable = 12;
    println!("Original value -> {}", mutable_variable);
    mutable_variable = 50;
    println!("Reassigned value -> {}", mutable_variable);

    // Variables can also be overwritten with shadowing.
    let mutable_variable = true;

    println!("Next Reassigned value -> {}", mutable_variable);


    /*
    * Literals and Operators
    *
    * - Integers, floats, characters, strings, booleans, and the unit type can all be
    *   expressed using literals
    * - Integers can, alternatively, be expressed using hexadecimal, octal, or binary notation
    *   using these prefixes respectively: 0x, 0o, 0b
    * - Underscores can be inserted in numeric literals to improve readability, eg. 1_000 is the
    *   same as 1000, and 0.00_001 is the same as 0.00001.
    * - We need to tell the compiler the type of the literal we use. For now, we'll use the u32 suffix
    *   to indicate that the literal is an unsigned 32-bit integer, and the i32 suffix to indicate 
    *   that it's a signed 32-bit integer.
    */

    // Integer addition
    println!("1 + 2 = {}",1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}",1i32 - 2);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}",!true);

    // Bitwise operations
    println!("00100 AND 0101 is {:04b}",0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
