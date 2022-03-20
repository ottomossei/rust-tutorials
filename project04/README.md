# 1. スタックとヒープ
 - スタック
   - last in, first out (LIFO)、push & pop
   - 変数が既知の固定サイズ→場所を参照することなく高速
 - ヒープ
   - allocating on the heap
   - サイズが可変のデータでOSのヒープ領域にメモリを確保、ポインタを返す
   - ポインタを経由するため、スタックより低速。メモリ開放を忘れるとメモリリークが発生

# 2. 所有権規則
 - Rustの各値は、所有者と呼ばれる変数と対応している
 - いかなる時も所有者は１つ
 - 所有者がスコープを外れると、値は破棄される

# 3. 整数型
整数型は、大きさが固定長であるため、スタックに積まれる
「値5をxに束縛する; それからxの値をコピーしてyに束縛する。」
→ 二つの変数(xとy)が存在し、両値とも5となる
```rust
let x = 5;
let y = x;
```

# 34. String型
String型は可変長なので、実行時にヒープ領域のメモリ確保
String型が終わると、メモリが開放される
```rust
fn main(){
    let s = String::from("hello"); // s有効
}// s無効
```
```rust
fn main(){
    let s = String::from("hello"); // s有効
}// s無効、自動でdrop関数が起動(free)
```

![This is an image]()

<img src="https://raw.githubusercontent.com/ottomossei/rust-tutorials/a25931ed4b7cc5daa7944174e37e898aae3f7ab0/project04/static/01.svg" width="400px">
