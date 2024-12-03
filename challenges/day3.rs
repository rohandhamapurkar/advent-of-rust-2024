// We need to find the nice and naughty kids for santa

// Each good deed is worth 1 point and each bad deed is worth 2 points
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
	let good_calc = good_deeds as f32 * GOOD_WEIGHT;
	let result = good_calc / (good_calc + (bad_deeds as f32 * BAD_WEIGHT));
	return result >= 0.75;
}


pub fn main(){
	println!("{}",is_nice(13, 2));
}
