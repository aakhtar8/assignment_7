// this will subtract two variables using the defintion in lib.rs
use a7q2::calculater::arthmetic_opertions::subtract;
fn main() {
	let variable_1 = 32.0;
	let variable_2 = 12.30;
	let output = subtract(variable_1, variable_2);
	println!("Difference of {} and {} is {}",variable_1, variable_2, output);
	
	}
