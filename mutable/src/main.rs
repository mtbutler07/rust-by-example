// By default, variables in rust are immutable.
//  meaning they cannot be changed once a value is bound to a name


fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    // variables can be made mutable by using the keyword `mut`
    let mut y = 7;
    println!("The value of y is: {}", y);
    y = 8;
    println!("The value of y is: {}", y);


    // Constants are always immutable and declared with the `const` keyword.alloc

    const MAX_POINTS: u32 = 100000;
    println!("The value of constant MAX_POINTS is: {}", MAX_POINTS);


}