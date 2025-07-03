// Welcome to rust for us(dummies) part - 2

// variables and mutability
//
// ===Variables===
// Variables are container which store values, like a fruitbasket contain fruits , like your body
// contains your stupid brain , like your home contains a dummy like you.

// if we have to create a variables we can create through a keyword "let"
// If you dont understand what is a keyword is -> Brother you need some recall pill we have
// discussed in in previous part.
//
// example of how you can create a variable
// let 'name of the variable' = value of it.
// When we write the name , make sure it should be on point , concise and explains what it do.
// ex: let we_have_monkey_eating_banana_and_number_of_banana_is = 5. -> this is verbose , dont do
// this
//
// instead do this :-
// let
// monkey_is_our_dummies_eating_banana_on_top_burj_khalifa_because_they_accepted_red_bull_challenge_to_eat_a_banana_on_burj_khalifa_just_for_some_money.God
// bless_our_poor_dummines = 5.
//
// Just kidding , No violence (just peace and violin.)
// Here how you do that :-
// let banana = 5
//
// here you noticed i didnt used spaces because i was following snake naming convention or snake
// case.
//
// like we use naming convention to create file name , similarly you use rust naming convention to
// write name of variables
//
// Before someone asks what is a naming convention -> Thats just means a rule or style(you can say) , you have to follow while adopting a new language
// for every language this naming convention changes.
// When i heard about it , i thought same -> why dont they standardise it , and make it universal
// for every language.
//
// but nonetheless , it doesnt matter.
//
// so rust naming convention is to follow snake_case (thats just the name of naming convention, my
// personal favorite is camelCase because i came from java background.)
//
// so in snake case , you dont use capital letter in the word(except for constant, thats just
// the rule for naming , you could deny that , if you want but rust will just generate warning to
// use snake_case)
//
//
fn main() {
    // This statement is called declaration , we declare our variable in the scope of main
    // function.
    // scope is just {} -> these curly brackets
    // if something is in scope that means that , it is available for you to be used in the curly
    // brackets.
    //
    // currently this variable doesnt have any value so if you print it , it wont show anything on
    // the terminal and print error.
    //
    // one point:- make sure to use semicolon to end the statement.
    let banana_for_dummies;
    // println!("{banana_for_dummies}"); // note:- this will produce error because println need
    // something to print since it hasnt anything in it , it will produce error.

    // This is called initialization , meaning now it have some value in it.
    // Now you print it , it will have some value in it.
    banana_for_dummies = 10;

    println!("{banana_for_dummies}"); // will print 10 on the command line(CLI in short)

    // ==== Mutability =======
    // Now dummies focus as this is a very important topic.
    // by default , a variable in rust is immutable.
    // what does mean by "immutable"?
    // it means that , the assigned value of variable cannot be changed.
    //
    // understand this :- when we initialized the variable we assigned it a value and that value
    // cannot be changed by default , that mean , if you reinitialize a variable , it wont work and
    // produce error.
    //
    // banana_for_dummies = 5; -> this will produce error.
    //
    // so you will be asking, why when we declared the variable and initialized it ,it did not
    // produced error ?
    // because the first time we declare and initialize it , rust compiler was smart enough to tell
    // us that it was empty and then we added value to it.
    // So second time we tried to change value of the variable , rust compiler was again smart enough to
    // tell us that it has a value assigned to it and cannot change it.
    //
    // what is rust compiler ?
    // rust compiler is a program that turn our code into macine code.
    //
    // note :- when we talk about rust ,we are actually talking about its compiler.
    //
    // So question arises how we will change the value of the variable
    // we can do so by making the variable mutable , meaning now we can change its value
    //
    // so how are we going to do that ?
    // we can do so using "mut" keyword , we will use this keyword when we are delacring the
    // variable to make it mutable.
    //
    // like this :-
    // let mut {name of variable} ;
    // {name of variable} = value;
    //
    // also note we can declare and initialize it in one line , like we done it earlier.
    // let mut {name of variable} = 5;
    //
    // now we can change the variable like this
    // {name of variable} = new value;
    //
    // ex:-
    let mut apples_for_dummies = 19; // declared it with mut keyword and initialized it with
    // 19
    println!("{apples_for_dummies}");

    // reinitialize it or you can say change the value of it.
    apples_for_dummies = 5;
    println!("{apples_for_dummies}");

    // Thats it for this chapter.
    // Thank you for reading this.
}
