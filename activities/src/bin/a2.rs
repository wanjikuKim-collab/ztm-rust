// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result


// * Use a function to add two numbers together
fn add(a: i32, b:i32)->i32 {
    a + b
}

// * Use a function to display the result
fn display_result(){
    println!("The sum of a and b is: {:?}", add(5, 6));
}
fn main() {
    
}
