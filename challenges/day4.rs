pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)] // needed for tests
pub enum Niceness {
	Nice(u32),
	Naughty
}

pub struct Kid {
	pub name: String,
	pub niceness: Niceness
}

impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
		if Kid::is_nice(good_deeds,bad_deeds) {
			return Kid { name: name, niceness: Niceness::Nice(good_deeds) }
		} else {
			return Kid { name: name, niceness: Niceness::Naughty }
		}
    }

	pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
		if good_deeds == 0 && bad_deeds == 0 {
			return false;
		}
	
		let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
		let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;
	
		let ratio = good_deeds / (good_deeds + bad_deeds);
	
		ratio >= 0.75
	}
}

fn main() {
	
}
