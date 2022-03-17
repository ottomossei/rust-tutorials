#[allow(dead_code)]
pub fn main() {
    println!("Example 02");
    // while_loop();
    loop_break();
    println!();
}

#[allow(dead_code)]
fn loopping() {
    println!("Example 02-01");

    loop {
        println!("again!");
    }
}

fn loop_break() {
    println!("Example 02-02");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };

    println!("result : {}", result);
}
