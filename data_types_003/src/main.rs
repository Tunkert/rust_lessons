fn main() {
    /*
    Scalar types - integers, floating point numbers, booleans, characters
    scalar types represent one value
     */

    /*
    Integers - a number without a fractional component
    types of variables:
    -(2^n-1) to 2^(n-1) - 1
    8-bit i8 u8 -128 to 127 0 to 255
    16-bit i16 u16
    32-bit i32 u32
    64-bit i64 u64
    128-bit i128 u128
    arch isize usize (64 bit if you're on a 64 bit system and 32 bit if you're on a 32 bit system)
    default is i32
     */

    /*
    Number literals
    Decimal 3_500 (3,500)
    Hex 0xab
    Octal 0o12
    Binary 0b0000_0101
    Byte (u8) b'A'
     */

    // integer overflow
    // let my_num: u8 = 255; // if I put in 256 - this is going to cause an error and it won't compile
    // println!("This is the max value for a u8: {my_num}.");

    /*
    floats
    f32 - single-precision float
    f64 - double-precision float
     */

    /*
    booleans
    let is_male = true; // implicit type notation
    let is_teacher: bool = true; // explicit type annotation
     */

    /*
    character type
    let grade = 'A'; // implicit type - notice single quotes
    let your_grade: char = 'C'; // explicit type annotation
     */

    /*
    tuples
    tuples have a fixed length
    types in tuples don't have to be the same
    let my_tup: (i32, f64, &str) = (7, 3,141516789, "Tim");
     */

    // use pattern matching with tuples
    // let tup = (400, 2.718, "Timothy Unkert");
    // let (_a, _b, c) = tup; // put underscores for unused variables
    // println!("My name is {c}.");

    // arrays
    let nums = [1, 2, 3, 4, 5];
    let first = nums[0];
    let last = nums[4];
    println!("The first num is {first}.");
    println!("The last num is {last}.");

    // print the length of the array
    let nums_length = nums.len();
    println!("The length of the nums array is {nums_length}.");

}
