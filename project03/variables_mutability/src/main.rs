mod example01_let;
mod example02_mut;
mod example03_const;
mod example04_shadowing;

fn main() {
    // let is immutable
    example01_let::main();
    // mut is mutable
    example02_mut::main();
    // const
    example03_const::main();
    // shadowing
    example04_shadowing::main();
}
