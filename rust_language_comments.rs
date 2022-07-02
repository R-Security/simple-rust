// Simple Beginner Program with RUST
// Author : Raed Ahsan
// Creation Date : 01/07/2022
// Time : 2:41AM

// Step 1 : Creation of the function.
// fn main_function() {
// 	println!("RUST training from R-Security. ");
// }

// Taking user input from RUST language.

fn main() {
	let mut name  = String::new();
	println!("Enter your good name: ");
	let bytes = std::io::stdin().read_line(&mut name).unwrap();
	println!("It's pleasant to meet you {}", name);
	//println!("Number of bytes in your name = {}", bytes); // uncomment this line if you want to see the number of bytes in your name

}
