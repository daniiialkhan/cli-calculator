#![allow(unused_imports)]
#![allow(unused_variables)]

use std::panic::catch_unwind;

pub mod input{
	use std::io;
	pub fn input_num() -> f64 {
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

	pub fn input_op() -> char {
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
}
pub mod calculation{
	pub fn add(num1: f64, num2: f64) -> f64 {
		let result = num1 + num2;
		println!("Result: {:.1}", result);
		result
	}
	pub fn sub(num1: f64, num2: f64) -> f64 {
		let result = num1 - num2;
		println!("Result: {:.1}", result);
		result
	}
	pub fn mul(num1: f64, num2: f64) -> f64 {
		let result = num1 * num2;
		println!("Result: {:.1}", result);
		result
	}
	pub fn div(num1: f64, num2: f64) -> f64 {
		if num2 == 0.0 {
			println!("Cannot divide by zero");
			return 0.0;
		}
		let result = num1 / num2;
		println!("Result: {:.1}", result);
		result
	}
}

#[cfg(test)]
mod tests {
    use super::*;
	use calculation::{add, sub, mul, div};
	use rand::Rng;
	use std::io::Write;
	use std::error::Error;
	use std::fs::File;
	use std::io::prelude::*;
	use csv::Writer;
	use std::time::{SystemTime, UNIX_EPOCH};

	const ITR: u32 = 50; // Number of iterations of each random test

	fn get_timestamp() -> u64 {
		let now = SystemTime::now();
		let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
		since_epoch.as_secs()
	}

	fn random_num() -> f64 {
		let mut rng = rand::thread_rng();
		rng.gen_range(0.0..100.0)
	}

	fn create_file(filename: &str) -> Result<File, std::io::Error> {
		let timestamp = get_timestamp();

		match std::fs::create_dir_all(format!("./test_results/{}", timestamp)) {
			Ok(()) => {
				println!("Directory created successfully");
				let filepath = format!("./test_results/{}/{}.csv", timestamp, filename);
				match File::create(filepath) {
					Ok(file) => {
						println!("File created successfully");
						Ok(file)
					},
					Err(e) => {
						println!("Error creating file: {}", e);
						Err(e)
					},
				}

			},
			Err(e) => {
				println!("Error creating directory: {}", e);
				Err(e)
			},
		}

	}

	////////////////////////////// Add tests //////////////////////////////
	/// 
    #[test]
    fn basic_add_test() {
        let result = add(2.0, 3.0);
        assert_eq!(result, 5.0);
    }

	#[test]
    fn random_add_test() {
		let file = match create_file("random_add_test") {
			Ok(file) => file,
			Err(e) => return,
		};
		let mut writer = Writer::from_writer(file);
		// write headers
		writer.write_record(&["Iteration", "num1", "num2", "result", "Status"]).expect("Unable to write record");

		for i in 0..ITR {
			let num1 = random_num();
			let num2 = random_num();
			let result = add(num1, num2);
			// assert_eq!(result, num1+num2, "Random Test Failed for iteration: {}, with num1: {:.1} and num2: {:.1}", i, num1, num2);
			let check = catch_unwind(|| assert_eq!(result, num1+num2, "Random Test Failed for iteration: {}, with num1: {:.1} and num2: {:.1}", i, num1, num2));
			if check.is_err() {
				// println!("Random Test Passed for iteration: {}, with num1: {:.1} and num2: {:.1}", i, num1, num2);
				writer.write_record(&[i.to_string(), num1.to_string(), num2.to_string(), result.to_string(), "FAIL".to_string()]).expect("Unable to write record");
				assert!(false, "Random Test Failed for iteration: {}, with num1: {:.1} and num2: {:.1}", i, num1, num2);
			}
			else {
				writer.write_record(&[i.to_string(), num1.to_string(), num2.to_string(), result.to_string(), "PASS".to_string()]).expect("Unable to write record");
				writer.flush().expect("Unable to flush");
			}
		}
	}

	////////////////////////////// Sub tests //////////////////////////////
	
    #[test]
    fn basic_sub_test() {
        let result = sub(5.0, 3.0);
        assert_eq!(result, 2.0);
    }

	#[test]
    fn random_sub_test() {
		let file = match create_file("random_sub_test") {
			Ok(file) => file,
			Err(e) => return,
		};
		let mut writer = Writer::from_writer(file);
		// write headers
		writer.write_record(&["Iteration", "num1", "num2", "result", "Status"]).expect("Unable to write record");

		for i in 0..ITR {
			let num1 = random_num();
			let num2 = random_num();
			let result = sub(num1, num2);
			
			let check = catch_unwind(|| assert_eq!(result, num1-num2, "Random Test Failed for iteration: {}, with num1: {:.1} and num2: {:.1}", i, num1, num2));
			if check.is_err() {
				// println!("Random Test Passed for iteration: {}, with num1: {:.1} and num2: {:.1}", i, num1, num2);
				writer.write_record(&[i.to_string(), num1.to_string(), num2.to_string(), result.to_string(), "FAIL".to_string()]).expect("Unable to write record");
				assert!(false, "Random Test Failed for iteration: {}, with num1: {:.1} and num2: {:.1}", i, num1, num2);
			}
			else {
				writer.write_record(&[i.to_string(), num1.to_string(), num2.to_string(), result.to_string(), "PASS".to_string()]).expect("Unable to write record");
				writer.flush().expect("Unable to flush");
			}
			
		}
	}
	////////////////////////////// Mul tests //////////////////////////////


    #[test]
    fn basic_mul_test() {
        let result = mul(2.0, 3.0);
        assert_eq!(result, 6.0);
    }

	#[test]
    fn random_mul_test() {
		let file = match create_file("random_mul_test") {
			Ok(file) => file,
			Err(e) => return,
		};
		let mut writer = Writer::from_writer(file);
		// write headers
		writer.write_record(&["Iteration", "num1", "num2", "result", "Status"]).expect("Unable to write record");

		for i in 0..ITR {
			let num1 = random_num();
			let num2 = random_num();
			let result = mul(num1, num2);

			let check = catch_unwind(|| assert_eq!(result, num1*num2, "Random Test Failed for iteration: {}, with num1: {:.1} and num2: {:.1}", i, num1, num2));
			if check.is_err() {
				// println!("Random Test Passed for iteration: {}, with num1: {:.1} and num2: {:.1}", i, num1, num2);
				writer.write_record(&[i.to_string(), num1.to_string(), num2.to_string(), result.to_string(), "FAIL".to_string()]).expect("Unable to write record");
				assert!(false, "Random Test Failed for iteration: {}, with num1: {:.1} and num2: {:.1}", i, num1, num2);
			}
			else {
				writer.write_record(&[i.to_string(), num1.to_string(), num2.to_string(), result.to_string(), "PASS".to_string()]).expect("Unable to write record");
				writer.flush().expect("Unable to flush");
			}
		}
	}

	////////////////////////////// Div tests //////////////////////////////
	
    #[test]
    fn basic_div_test() {
        let result = div(5.0, 5.0);
        assert_eq!(result, 1.0);
    }

	#[test]
    fn random_div_test() {
		let file = match create_file("random_div_test") {
			Ok(file) => file,
			Err(e) => return,
		};
		let mut writer = Writer::from_writer(file);
		// write headers
		writer.write_record(&["Iteration", "num1", "num2", "result", "Status"]).expect("Unable to write record");

		for i in 0..ITR {
			let num1 = random_num();
			let num2 = random_num();
			let result = div(num1, num2);

			if num2 == 0.0 {continue;} // skip iteration if num2 is 0
			
			let check = catch_unwind(|| assert_eq!(result, num1/num2, "Random Test Failed for iteration: {}, with num1: {:.1} and num2: {:.1}", i, num1, num2));
			if check.is_err() {
				// println!("Random Test Passed for iteration: {}, with num1: {:.1} and num2: {:.1}", i, num1, num2);
				writer.write_record(&[i.to_string(), num1.to_string(), num2.to_string(), result.to_string(), "FAIL".to_string()]).expect("Unable to write record");
				assert!(false, "Random Test Failed for iteration: {}, with num1: {:.1} and num2: {:.1}", i, num1, num2);
			}
			else {
				writer.write_record(&[i.to_string(), num1.to_string(), num2.to_string(), result.to_string(), "PASS".to_string()]).expect("Unable to write record");
				writer.flush().expect("Unable to flush");
			}
		}
	}
}