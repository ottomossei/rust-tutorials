#[allow(dead_code)]
pub fn main01() {
    println!("Example 04-01");
    let a = [1, 2, 3, 4, 5];

    let mut first = a[0];
    let second = a[1];
    println!("first : {}", first);
    println!("second : {}", second);

    first = 9;
    println!("first : {}, a[0]: {}", first, a[0]);
}

#[allow(dead_code)]
pub fn main02() {
    println!("Example 04-02");
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element); //panic
}
