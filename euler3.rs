/* 
The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143 ?
*/

fn main() {
    let number: u64 = 600851475143;
    for i in (3..number/2).step_by(2) {
        if number % i == 0 && is_prime(number / i) {
            println!("{}", number / i);
            break;
        }
    }
}

fn is_prime(number: u64) -> bool {
    for i in (3..number/2).step_by(2) {
        if number % i == 0 || number % 2 == 0 {
            return false;
        }
    }
    true
}
