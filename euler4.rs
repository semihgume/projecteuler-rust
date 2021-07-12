/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
Find the largest palindrome made from the product of two 3-digit numbers.
*/

fn main() {
    let mut result = 0;
    for i in (100..=990).rev().step_by(11) {
        for j in (100..=999).rev() {
            let multi = i * j;
            if multi > result && palindrom(multi) {
                result = multi;
                break;
            } else if result > multi {
                break;
            }
        }
    }
    println!("Result: {:?}",result);
}

fn palindrom(number: i32) -> bool {
	return number.to_string() == number.to_string().chars().rev().collect::<String>();
}


