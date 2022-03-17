#[allow(dead_code)]
pub fn main() {
    println!("Example 01");
    if_else();
    if_in_let();
    println!();
}

fn if_else() {
    println!("Example 01-01");
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_let() {
    println!("Example 01-02");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number : {}", number)
}
