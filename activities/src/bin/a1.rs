// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn first_name (first_name: String) -> String {
    first_name
}

fn last_name (last_name: String) -> String {
    last_name
}

fn main() {
    println!("{} {}", first_name(String::from("Faith")), last_name(String::from("Kimani")));
}
