//! RNG from Uniform Distribution

use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::cmp; 

/// Global seed
static SEED: AtomicU32 = AtomicU32::new(0);

/// Pseudorandom seed using the current time
fn rand_seed() -> u32 {
	let seed = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_nanos() as u32;
	return seed;
}

/// `n` pseudorandom numbers from [0, 1] 
/// 
/// Using the Linear Congruential Generator (LCG)
/// Lewis, Goodman, and Miller in 1969 
/// 
/// C-like Parameters from
/// [wikipedia](https://en.wikipedia.org/wiki/Linear_congruential_generator)
/// 
pub fn rand(n:u64) -> Vec<f64> {

	// If the first time and the SEED is not set
	let _ = SEED.compare_exchange(
		0, 
		rand_seed(),
		Ordering::Relaxed,
		Ordering::Relaxed
	);

	let a: u32 = 22_695_477;
	let c: u32 = 1;

	// 32-bit SEED converted to 31-bit
	let mut x: u32 = SEED.load(Ordering::Relaxed) & 0x7FFFFFFF;
	let mut r: Vec<f64> = Vec::new();
	for _ in 0..n {
		// next seed
		x = x.wrapping_mul(a).wrapping_add(c) & 0x7FFFFFFF;
		SEED.store(x,Ordering::Relaxed);

		// map to [0 1]
		// 1. shift the 31-bit by 16 => 15-bit left
		// 2. max = 2^15 - 1 = 32,767
		// 3. x / max to map to [0 1]
		r.push((x >> 16) as f64 / 32767.0);
	}
	return r; 
}

/// `One` pseudorandom number from [0, 1]
pub fn rand1() -> f64 {	
	let r = rand(1);
	return  r[0];
}

/// `n` pseudorandom integers from [a, b]
pub fn randi(start:i64, end:i64, n:u64) -> Vec<i64> {
	let min = cmp::min(start, end) as f64;
	let max = cmp::max(start, end) as f64;
	let length = max - min;
	let r = rand(n);
	return r.into_iter().map(|x| (min + length * x) as i64).collect();
}

/// `One` pseudorandom integer from [a, b]
pub fn randi1(start:i64, end:i64) -> i64 {
	return randi(start, end, 1)[0];
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Check one pseudorandom number is between 0 and 1
	#[test]
	fn check_rand1(){
		let n = 10_000;
		for _ in 0..n {
			let r = rand1();
			assert!(r >= 0.);
			assert!(r <= 1.);
		}
	}

	/// Check the mean (mu) and variance (var) of pseudorandom numbers
	#[test]
	fn check_rand() {
		let r: Vec<f64> = rand(1000_000);
		let count: usize = r.iter().len();
		let mu: f64 = r.iter().sum::<f64>() / count as f64;
		let var = r.iter()
			.map(|v| {
				let distance = *v - mu;
				distance * distance
			})
			.sum::<f64>() / count as f64;
		
		// mean error
		let er_mu = (0.5 - (mu * 1e4).round() / 1e4).abs();
		assert!(er_mu < 0.01);
		
		// variance error (theoretical variance: 1/12 ~ 0.0833)
		let er_var = (1./12. - (var * 1e4).round() / 1e4).abs();	
		assert!(er_var < 0.001);
	}

	/// Check pseudorandom integers are between the start and end args
	#[test]
	fn check_randi(){
		let a = -10;
		let b = 10;
		let n = 10_000;
		let r = randi(a, b, n);
		for i in 0..n {
			assert!((a..=b).contains(&r[i as usize]));
		}
	}

	/// Check one pseudorandom integer is between the start and end args
	#[test]
	fn check_randi1(){
		let a = -10;
		let b = 10;
		let n = 10_000;
		for _ in 0..n {
			let r = randi1(a, b);
			assert!((a..=b).contains(&r));
		}
	}	
}