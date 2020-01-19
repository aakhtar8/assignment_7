// calculater module to perform all kind of calculations
mod calculater {
	// sub module for arthmetic_operations
	pub mod arthmetic_opertions {
		pub fn add (input_1: f32, input_2: f32) -> f32 {
			let result = input_1 + input_2;
			result
			
			
			}

		
		}
	}
	
fn main() {
	let variable_1 = 11.2;
	let variable_2 = 34.0;
	println!("sum of {}, and {}, will be calculated using calculater module",variable_1, variable_2);
	// calling the module in the main function
	let output = crate::calculater::arthmetic_opertions::add(variable_1, variable_2);
	println!("sum of the given numbers is {}",output);
	}
