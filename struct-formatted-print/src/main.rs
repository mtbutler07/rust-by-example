use std::fmt;

fn main() { 
    
    /*
     * All types which want to use std::fmt formatting traits require an implementation to be
     * printable. Automatic implementations are only provided for types such as in the std library.
     * All others must be manually implemented somehow.
     *
     * The fmt::Debug trait makes this very straightforward. All types can derive (automatically create)
     * the fmt::Debug implementation. This is not true for fmt::Display which must be manually implemented.
     */

    // The following structure can not be printed with fmt::Display or fmt::Debug automatically.
    // For debug printing, we just use the `derive` attribute to make the struct printable using fmt::Debug `{:?}`
    #[derive(Debug)]
    struct Structure(i32);

    println!("{:?}", Structure(3));

    // Derive the fmt::Debug implementation for the struct
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Marcus";
    let age = 27;
    let marcus = Person { name, age };

    // Pretty print using #
    println!("{:#?}", marcus);

    /*
    * To help clean up the appearance of fmt::Debug formatting, it can be advantageous to 
    * manually implement fmt::Display, which uses the {} print marker.
    */

    // Tuple struct named SingleStructure that contains a single i32
    struct SingleStructure(i32);

    // Tuple struct named DoubleStructure that contains a two i32
    struct DoubleStructure(i32, i32);

    // Manually implement the trait fmt::Display in order to use the {} print marker
    impl fmt::Display for SingleStructure {

        // The `fmt` trait requires implementation using this exact signature
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    // Manually implement the trait fmt::Display in order to use the {} print marker
    impl fmt::Display for DoubleStructure {

        // The `fmt` trait requires implementation using this exact signature
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {}", self.0, self.1)
        }
    }

    let single_structure = SingleStructure(4);
    let double_structure = DoubleStructure(4, 10);

    println!("Single Structure Format Printing -> {}",single_structure);
    println!("Double Structure Format Printing -> {}",double_structure);


    // Let's do format printing for more complex structures
    struct Point2D {
        x: f64,
        y: f64,
    }

    // Manually implementing the fmt::Display trait for Point2D
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result {
            write!(f, "({:.3}, {:.3})", self.x, self.y)
        }
    }

    let coordinates = Point2D{x:7.34588, y:9.2};

    println!("{}",coordinates);
}