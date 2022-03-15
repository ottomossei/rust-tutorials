
## 1.1 Integer Types  

|  Length  |  Signed  |  Unsigned  |
| ---- | ---- | ---- |
|  8-bit  |  i8  |  u8  |
|  16-bit  |  i16  |  u16  |
|  32-bit  |  i32  |  u32  |
|  64-bit  |  i64  |  u64  |
|  128-bit  |  i128  |  u128  |
|  arch  |  isize  |  usize  |

Default type is i32.

## 1.2 Integer Literals

|  Number literals	  |  Example  |
| ---- | ---- |
|  Decimal  |  98_222  |
|  Hex  |  0xff  |
|  Octal  |  0x77  |
|  Binary  |  0b1111_0000  |
|  Byte (u8 only)  |  b'A'  |

## 2. Floating-Point Types
```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```
Default type is f64.


## 3. The Boolean Type
```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

## 4. The Character Type
```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```
 Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. 



## 5. Compound Types
## 5.1 The Tuple Type
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```
```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```
