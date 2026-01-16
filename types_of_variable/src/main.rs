// Welcome to rust for us(dummies) - part 4
// topic: Type of variable in rust

// Being a statically typed language, Rust requires that every variable's type is known at compile time.

// we usually don't write it, or I say type annotate it because rust compiler handles the work of annotating
// variables for us.
// Handling type annotations for the variable by the rust compiler is known as type inference.

// Type annotations
// To specify a variable type , we annotate it like this:
// let no_banana_for_us_this_time: bool = true;
// we are specifying this variable of boolean type , meaning it can have two values either true or false.

fn main() {
    // One more example of type annotated variable
    // let i_need_banana_for_ourselves: bool = true;

    // Types of variable in rust

    // Scalar type
    // that type of variable which contains only one type of value.
    // like how your body contain a one stupid brain of yours.

    // ex:
    // let monkey_age = 32; // integer

    // rust has four scalar types of variables
    // integers, floating-point numbers, booleans, and characters.

    // Here is the list of them:
    // let x = 64; // Integer
    // let x = 5.2; // Float
    // let x = true; // Boolean
    // let x = 'f'; // Char

    // Compound type
    // Compound types group multiple values into a single type.
    // like how fruit basket contains different types of fruits.

    // Rust primarily has two types of compound type of variable:
    // 1. tuples -> tuples are the types which can store multiple types of variable of different type, or I can say group multiple types of variable of different types.
    // let tuples = (64, true,'c', 6.4);

    // 2. arrays -> arrays are the types which store multiples values of variable of same type.
    // let array = [64,35,34,23,64];

    // Thanks for reading this.

}
