// This is a comment, and it should not be included in the code.
// Just to communicate through code.
/*
* Multiple line comment.
*/

// This is main function where our code run and every project has one of these.
// Without these function the code won't run and compile. Compile means -> Transforming your code
// in binary language which machine could understand.
// fn means -> function, and it is a keyword.
// keyword are reserved words by the language.
fn main() {
    // This helps us to print "Hello world" on the screen.
    // print! -> is a macro -> if you come from background where you were using OpenOffice then you
    // will be familiar with this.
    // A macro is a predefined set of instruction performed by the program when we use it.
    print!("Hello World!"); // adding semicolon is optional -> but i think , it is good to use a
    // semicolon , it provides us a feeling that a statement has ended.

    // print macro + a new line = println! macro -> someone from c/c++/java background could
    // understand this.
    println!("After this line will generate a new line.");

    // Now we have two more print macros but they are used for error handling and error print.
    // error handling means that you are handling your error and prevent your program from
    // crashing/panicking!

    // used to print error message and progress message only.
    eprint!("Rust is panicking because a seagull has arrived!");

    println!("Intense music playing...");

    // eprint! + new line = eprintln
    eprintln!("Time to hide under the rocks!");
    println!("Safe...");

    // We have some additional properties of these four macros
    // adding a {} between the "" in those four macros can help us to add extra content in it.
    // This is a bad example because you can add 'Rust' between the words without needing {}.
    // Rust compiler will generate a warning for that.
    println!("Seagull has gone , Time for {} to come out.", "Rust");

    println!("Happy ending!");
}