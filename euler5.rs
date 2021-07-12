/*
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

fn main() {
	let mut result = 2520;
	let mut counter = 0;
	'outer: loop {
		for i in 1..21 {
		 	if result % i == 0 {
		 		counter = counter + 1;
		 		if counter == 20 {
		 			break 'outer;
		 		}
		 	} else {
		 		result = result + 2520;
		 		counter = 0;
		 		break;
		 	}
		}
	}
	println!("Result: {:?}",result);
}
