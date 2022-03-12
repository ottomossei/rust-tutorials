
# 1. Rust

## 1.1. Installation

skip


## 1.2. Hello world
```rust
fn main(){
    //println! calls a Rust macro. 
    println!("Hello, world");
}
```
rustc  is similar to gcc or clang.

```sh
rustc main.rs
./main
Hello, world!
```

## 1.3. Hello Cargo
Check cargo version
```sh
cargo --version
```

Basic usage
```sh
cargo new ${project_name}
cargo run // = cargo build && ./target/debug/${project_name}
Hello, world!
```

`cargo check` quickly checks your code to make sure it compiles but doesn’t produce an executable.
```sh
cargo check
```

If you release projects, execute `cargo build --release`.
This command compile it with optimizations but it takes time to finish compiling.




