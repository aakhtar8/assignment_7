pub mod calculater {
	// sub module for probablity calculation
	pub mod probabilty {
		pub fn factorial (mut input: u32) -> u32 {
			let mut result: u32 = 1;
			while input > 1 {
				result = result * input;
				input = input - 1;
				}
			result
			}
		}
	}
pub use calculater::probabilty::factorial;
