# 1. 基本
## 1.1. スタックとヒープ
 - スタック
   - last in, first out (LIFO)、push & pop
   - 変数が既知の固定サイズ→場所を参照することなく高速
 - ヒープ
   - allocating on the heap
   - サイズが可変のデータでOSのヒープ領域にメモリを確保、ポインタを返す
   - ポインタを経由するため、スタックより低速。メモリ開放を忘れるとメモリリークが発生

## 1.2. 所有権規則
 - Rustの各値は、所有者と呼ばれる変数と対応している
 - いかなる時も所有者は１つ
 - 所有者がスコープを外れると、値は破棄される

## 1.3. 整数型
整数型は、大きさが固定長であるため、スタックに積まれる  
「値5をxに束縛する; それからxの値をコピーしてyに束縛する。」  
→ 二つの変数(xとy)が存在し、両値とも5となる
```rust
let x = 5;
let y = x;
```

## 1.4. String型
String型は可変長なので、実行時にヒープ領域のメモリ確保  
String型が終わると、メモリが開放される
```rust
fn main(){
    let s = String::from("hello"); // sメモリ確保
}// sメモリ開放、自動でdrop関数が起動(free)
```
```rust
fn main(){
    let s1 = String::from("hello"); // s1メモリ確保, 【image01】
    let s2 = s1; // s1からs2へmove、【image02】
    println!("{}, world!", s1); // コンパイルエラー、s1はもう使用不可
}// s2メモリ解放
```

image01  
スタック上にヒープ領域のポインタ(ptr)、値の大きさ(len)、値の許容量(capacity)
<img src="https://raw.githubusercontent.com/ottomossei/rust-tutorials/a25931ed4b7cc5daa7944174e37e898aae3f7ab0/project04/static/01.svg" width="400px">

image02  
ポインタがコピーされ、実値（ヒープ領域）を再生成することはない  
shallow copyに近いが、s1は変数ごと無効となる。これを`move`と呼ぶ  
<img src="https://raw.githubusercontent.com/ottomossei/rust-tutorials/3152e1ea79e64782c8ebcf635176ada7e6b769cb/project04/static/04.svg" width="400px">  

## 1.5. clone
ヒープ領域のメモリをdeep copyする
```rust
fn main(){
    let s1 = String::from("hello");
    let s2 = s1.clone();
}
```

## 1.6. 所有権と関数
```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);// sの値が関数へmove、sは以後無効
    let x = 5;
    makes_copy(x);// xも関数にmove、しかしスタックなので今後も有効
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);// sがsome_thingとしてmovedされ使用される
}// some_string(=s)のメモリ解放

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
```

## 1.7. 戻り値とスコープ
```rust
fn main() {
    let s1 = gives_ownership(); // 戻り値がs1へmoveされる
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);// s2が関数へmoveされ、戻り値がs3にmoveする
} // s1, s3がdrop

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string //呼び出し元へmove
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
```
所有権を取り、またその所有権を戻す、ということを全ての関数で行なうのは冗長  
そのため、タプルで複数の値を返すことで煩雑さを軽減できる。  
この概念に対する機能として、`参照`がある
```rust
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
```