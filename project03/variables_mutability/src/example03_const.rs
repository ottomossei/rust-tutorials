pub fn main() {
    println!("Example 03");
    // Do NOT use mut.
    // Constants are valid throughout the duration of the program's run,
    // within the defined scope.
    // Rust's constant naming convention is to use all uppercase letters.
    const X: u32 = 100_1000;
    println!("X is {}", X);
}
