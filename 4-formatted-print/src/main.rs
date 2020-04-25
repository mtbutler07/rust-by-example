
fn print_pi(val: f32){
    println!("{:.3}", val)
}


fn main() {

    /* Printing in rust is handeled by a series of macros that are defined in the standard library (std::fmt)
    *
    *  - format!    = Write formatted text to a String
    *  - print!     = Same as format! but the text is printed to the console (io::stdout)
    *  - println!   = Same as print! but a newline is appended
    *  - eprint!    = Same as format! but the text is printed to the standard error (io::stderr)
    *  - eprintln!  = Same as eprint! but a newline is appended 
    *
    */

    // The '{}' will be replaced with any argument passed. AKA "stringified".
    // Without a suffix, the variable 'x' becomes an i32.
    let x = 31;
    println!("{} days", x );

    // Multiple arguments
    let x = x + 1;
    let y:i32 = x * 2;
    println!("{} days before {} days after", x, y );

    let sep = "------".repeat(10);
    println!("{}", sep);
    
    // Positional formatting works also!
    println!("{0}, this is {1}. {1}, this is {0}","Alice", "Bob");

    // Names arguments
    println!("{subject} {verb} {object}",object="the lazy dog.", subject="The quick brown fox", verb="jumps over");

    // Special formatting
    println!("{} in {:b} people know binary, the other half doesn't", 1,2);

    println!("{}", sep);

    // Wait, did that just print out integers in binary.....
    println!("{0}.{1}.{2}.{3} -> {0:b}.{1:b}.{2:b}.{3:b}", 192 ,168 ,1 ,2);

    // it does!, I wonder if it can format to show the leading zeros
    println!("{0}.{1}.{2}.{3} -> {0:08b}.{1:08b}.{2:08b}.{3:08b}", 192 ,168 ,1 ,2);

    // Neat! lets try subnet masks
    println!("{0}.{0}.{0}.{0} -> {0:08b}.{0:08b}.{0:08b}.{0:08b}", 255);

    println!("{}", sep);

    // You can right-align text with a specified width. 
    println!("{number:>width$}", number=2, width=5);

    println!("{}", sep);

    // Cool, back to the IP addresses
    println!("{0} {1:>width$}","Address" ,"Binary", width=17);
    println!("{0}.{1}.{2}.{3} {:>width$} {0:08b}.{1:08b}.{2:08b}.{3:08b}", 192 ,168 ,1 ,2, width=10);
    println!("{0}.{1}.{2}.{3} {:>width$} {0:08b}.{1:08b}.{2:08b}.{3:08b}", 192 ,168 ,1 ,3, width=10);
    println!("{0}.{1}.{2}.{3} {:>width$} {0:08b}.{1:08b}.{2:08b}.{3:08b}", 192 ,168 ,1 ,4, width=10);

    println!("{}", sep);

    // Pi Macro Exercise
    let pi = 3.141562;
    print_pi(pi);
}   
