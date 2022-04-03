#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{} pixcels", area(&rect1));
    // debug
    println!("rect1 is {:?}", rect1);
}

fn area(rectagle: &Rectangle) -> u32 {
    rectagle.width * rectagle.height
}
