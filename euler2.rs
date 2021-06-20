fn main() {
    let mut vec = vec![1,2];
    let mut sum = 0; 
    loop {
        if vec[vec.len() - 1] + vec[vec.len() - 2] < 4000000 {
            vec.push(vec[vec.len() - 1] + vec[vec.len() - 2]);
            continue;
        }
        break;
    }
    for v in vec {
        if v % 2 == 0 {
            sum += v;
        }
    }
    println!("{}", sum);
}