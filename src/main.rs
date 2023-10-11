
use std::io;


fn input_num() -> f64 {
	let stdin = io::stdin();

	let input = loop {
		let mut input_str = String::new();
		
		match stdin.read_line(&mut input_str) {
			Ok(bytes) => {
				println!("DEBUG: {bytes} bytes read, input_str = {input_str}");
			}
			Err(e) => println!("Error in Input: {e}"),
		};
		if let Ok(num) = input_str.trim().parse::<f64>() {
			println!("DEBUG: num = {num}");
			break num;
		}
		println!("Invalid Input,\nInteger or Float needed,\n Please enter again: ");
	};
	input
}

fn input_op() -> char {
	let stdin = io::stdin();

	let input = loop {
		let mut input_str = String::new();

		match stdin.read_line(&mut input_str) {
			Ok(bytes) => {
				println!("DEBUG: {bytes} bytes read, input_str = {input_str}");
			}
			Err(e) => println!("Error in Input: {e}"),
		};
		
		if input_str.trim() == "+" || input_str.trim() == "-" || input_str.trim() == "*" || input_str.trim() == "/" {
			let input_char = input_str.trim().chars().next().unwrap();
			println!("DEBUG: input_char = {input_char}");
			break input_char;
		}
		println!("Invalid Input,\nOperator must be in [+, -, *, /]\n Please enter again: ");
	};
	input
}

fn add(num1: f64, num2: f64) {
	println!("Result: {}", num1 + num2);
}
fn sub(num1: f64, num2: f64) {
	println!("Result: {}", num1 - num2);
}
fn mul(num1: f64, num2: f64) {
	println!("Result: {}", num1 * num2);
}
fn div(num1: f64, num2: f64) {
	if num2 == 0.0 {
		println!("Cannot divide by zero");
		return;
	}
	println!("Result: {}", num1 / num2);
}
/*
	fn add_generic<T>(num1: T, num2: T){  //.. Can not add generic type as it is not possible to perform +*-/ operations on generic type
		println!("Result: {}", num1 + num2);
	}
	fn sub_generic<T>(num1: T, num2: T){
		println!("Result: {}", num1 - num2);
	}
	fn mul_generic<T>(num1: T, num2: T){
		println!("Result: {}", num1 * num2);
	}
	fn div_generic<T>(num1: T, num2: T){
		println!("Result: {}", num1 / num2);
	}
*/

fn main() {

	loop {

		println!("Enter num1: ");
		let num1: f64 = input_num();
	
		println!("Enter num2: ");
		let num2: f64 = input_num();
	
		println!("Enter operator: ");
		let op = input_op();
	
	
		match op {
			'+' => add(num1, num2),
			'-' => sub(num1, num2),
			'*' => mul(num1, num2),
			'/' => div(num1, num2),
			_ => println!("DEBUG: Invalid operator Check did not check the operator in function"),
		};

		println!("Do you want to make another calculation? (y/n)");
		
		let exit = loop{
			
			let mut choice_str = String::new();
			
			io::stdin().read_line(&mut choice_str).expect("Failed to read line");
			let choice_char = choice_str.trim().chars().next().unwrap();
			
			if choice_char == 'n' || choice_char == 'N' {
				break true;
			}
			else if choice_char == 'y' || choice_char == 'Y' {
				break false;
			}
			println!("Invalid Input,\nPlease enter again (y/n): ");
			
		};
		
		if exit {
			break;
		}

	}

}



