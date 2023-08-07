fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut numbers: Vec<i32> = args[1..].iter().map(|s| s.parse().unwrap()).collect();
    numbers.sort();
    if numbers.len() != 3 {
        println!("Usage: pythagorean a b c");
        return;
    }
    let (a, b, c) = (numbers[0], numbers[1], numbers[2]);
    if a * a + b * b == c * c {
        println!("This is a pythagorean triple");
    } else {
        println!("This is not a pythagorean triple");
    }
}
