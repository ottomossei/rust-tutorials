//  the function’s return type is specified too, as -> i32.
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // error
    // x + 1;
}

#[allow(dead_code)]
pub fn main() {
    println!("Example 03");
    let x = five();
    let y = plus_one(x);

    println!("x : {}", x);
    println!("y : {}", y);
}
