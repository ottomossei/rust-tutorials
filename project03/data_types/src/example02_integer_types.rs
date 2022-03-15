pub fn main() {
    println!("Example 02");
    let x: u16 = "2".parse().expect("Not a number!");

    println!(
        "x is {} bytes ({} bits)",
        std::mem::size_of_val(&x),
        std::mem::size_of_val(&x) * 8
    );
}
