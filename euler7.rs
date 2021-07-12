/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
What is the 10 001st prime number?
*/

fn main() {
	let mut counter = 6;
	let mut number = 14;
	loop {
		if is_prime(number) {
			counter = counter + 1;
		}
		if counter == 10001 {
			println!("Result: {:?}",number);
			break;
		}
		number = number + 1;
	}
}

fn is_prime(num: u64) -> bool {
	for i in (3..num/2).step_by(2) {
		if num % i == 0 || num % 2 == 0 {
			return false;
		}	
	}
	true
}
