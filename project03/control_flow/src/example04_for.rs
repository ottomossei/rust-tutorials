#[allow(dead_code)]
pub fn main() {
    println!("Example 04");
    for_basic();
    for_rev();
    println!();
}

fn for_basic() {
    println!("Example 04-01");
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("element : {}", element);
    }
}

fn for_rev() {
    println!("Example 04-02");
    for number in (1..4).rev() {
        println!("number : {}", number);
    }
}
