// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    let over_twenty_five = false;

    match over_twenty_five{
        true => println!("You're a grown up, you're over 25"),
        false => println!("Still a kid, enjoy being a kid")
    }
}
