// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    // * Use a mutable integer variable
    let mut num = 0;

    // * Use a loop statement
    loop{
        // * Print the variable within the loop statement
        println!("The number is: ", num);
        num = num + 1
        
        // * Use break to exit the loop
        if num == 4{
            break;
        }

    }
    
}
