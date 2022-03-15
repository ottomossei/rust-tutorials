pub fn main() {
    println!("Example 01");
    let x: u32 = "42".parse().expect("Not a number!");
    println!("x is {}", x);
}
