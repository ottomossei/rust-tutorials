pub fn main() {
    println!("Example 04_1");
    // Shadowing is different from marking a variable as mut,
    // because we’ll get a compile-time error if we try to reassign to this variable.
    let x = 1;
    let x = x + 1;
    let x = x * 2;
    println!("x is {}", x);

    println!("Example 04_2");
    //　Newly generated each time let is declared.
    let spaces = "   "; // spaces : String
    let spaces = spaces.len(); // spaces : Integer
    println!("spaces is {}", spaces);

    // if you use mut,
    // Generate warning: `variables_mutability`

    // let mut mut_spaces = "    "; // spaces : String
    // let mut_spaces = mut_spaces.len(); // spaces : Integer, Error
    // println!("spaces is {}", mut_spaces);
}
