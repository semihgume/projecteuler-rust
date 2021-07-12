fn main() {
	let mut squ = 0;
	let mut sum = 0;
	let diff: u32;
	for i in 1..101 {
		squ += i * i;
		sum += i;
	}
	sum = sum * sum;
	diff = sum - squ;
	println!("Karelerinin Toplamı: {}\nToplamlarının Karesi: {}\nFark: {}", squ, sum, diff);
}
