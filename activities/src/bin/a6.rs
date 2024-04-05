// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

fn main() {
    // * Use a mutable integer variable
    let mut counter = 5;

    // * Use a while statement
    while counter != 0{
        //Counts down from 5 to 1, displays the countdown
        println!("{}", counter);
        counter -= 1
    }
    //then prints "done!" when complete.
    println!("done");
}
