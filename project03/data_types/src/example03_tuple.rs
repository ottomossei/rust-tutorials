#[allow(dead_code)]
pub fn main01() {
    println!("Example 03-01");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!(
        "tupple 0 : {}, byte : {}",
        tup.0,
        std::mem::size_of_val(&tup.0)
    );

    println!(
        "tupple 1 : {}, byte : {}",
        tup.1,
        std::mem::size_of_val(&tup.1)
    );

    println!(
        "tupple 2 : {}, byte : {}",
        tup.2,
        std::mem::size_of_val(&tup.2)
    );

    println!("\n");
}

#[allow(dead_code)]
pub fn main02() {
    println!("Example 03-02");
    let tup = (500, 6.4, 1);
    println!(
        "tupple 0 : {}, byte : {}",
        tup.0,
        std::mem::size_of_val(&tup.0)
    );

    println!(
        "tupple 1 : {}, byte : {}",
        tup.1,
        std::mem::size_of_val(&tup.1)
    );

    println!(
        "tupple 2 : {}, byte : {}",
        tup.2,
        std::mem::size_of_val(&tup.2)
    );
}

#[allow(dead_code)]
pub fn main03() {
    println!("Example 03-03");
    // pattern 18.1
    let tup = (500, 6.4, 1);
    let (mut x, y, z) = tup;
    println!("x: {}, byte : {}", x, std::mem::size_of_val(&x));

    println!("y : {}, byte : {}", y, std::mem::size_of_val(&y));

    println!("z : {}, byte : {}", z, std::mem::size_of_val(&z));

    // 復習
    x = 400;
    println!("x: {}, tup.0: {}", x, tup.0);
}

#[allow(dead_code)]
pub fn main04() {
    println!("Example 03-04");
    // Error
    // let tup = (500);
    // println!("tup: {}", tup.0);

    let tup = (500,);
    println!("tup: {} \n", tup.0);
}
