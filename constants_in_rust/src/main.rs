// Welcome to rust for us(dummies) part - i dont know probably 3
// topic : constants in rust

// constants in rust can be created through 'const' keyword e.g.
// const MY_CONST : u32 = 6790;
//
// As you can see there is plenty of things happening here :-
// first , constant name is in capital letter snake case , it's the naming convention of rust .
// second , I have u32 , it means it is unsigned integer , we will cover these topics sometime
// later .
//
//
// so you might be asking why we need a const because we already have variable which is immutable
// by default and work as constants
//
// the main reason for that is that , variable can be mutated but const can never be mutated
// variable , by default is immutable but a constant is always immutable
//
// Creation of const
// to create a const , we need to use const keyword , gave a capital letter name , typer infer
// (it just mean adding a type to it like u8,i32,bool,etc.) and a value which should be assigned at
// compile time and not at run time , meaning value of them can not be dynamic
// e.g.
// let apples:u32 = 5;
// const ABSOLUTELY_MY_APPLES_AND_YOU_DONT_HAVE_RIGHTS_TO_EAT_THEM:u32 = 12 + apples;
//
// NOTE : The only thing you need make sure are that , 1. you need to type infer them ,
// 2.everything during initialization should be available at compile time and not dynamic.

fn main() {
    const APPLES: i32 = 5;
    const ABSOLUTELY_MY_APPLES_AND_YOU_DONT_HAVE_RIGHTS_TO_EAT_THEM: i32 = 23 + APPLES;
    println!("{ABSOLUTELY_MY_APPLES_AND_YOU_DONT_HAVE_RIGHTS_TO_EAT_THEM}");
}
