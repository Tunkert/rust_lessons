fn main() {
    println!("Hello, this will print to the console and print a new line.");
    // print!("This will print without a new line.");
    print!("But, I can create a new line with an escape character.\n");
    /*
    the ! calls a macro - without it would be a function, more about that later
    the main function, fn main is the entry point of the program
    rustc main.rs compiles the main.rs file to a binary which you can run with:
    ./main
    Notice this is a multiline comment
    // two slashes indicate a single line comment
    if someone tries to run the main binary without rust installed they can still run it
     */
}