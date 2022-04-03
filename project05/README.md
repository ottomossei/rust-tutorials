# 構造体の基本
## 定義とインスタンス化
構造体はタプル同様、一部を異なる型に出来る
```rust
// {}内をフィードと呼ぶ
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// インスタンス化
let mut user1 = User {
    email: String::from("user1@example.com"),
    username: String::from("user1"),
    active: true,
    sign_in_count: 1,
};

//　ドット記法(この際、インスタンスはmutable)
user1.email = String::from("anotheremail@example.com");
```

インスタンスを返す関数
```rust
// 一般的な関数
fn build_user(email: String, username: String) -> User {
    User { // emailとusernameを取得して、インスタンスを返す 
        email: email, // email
        username: username, // username 
        active: true,
        sign_in_count: 1,
    }
}

// さらなる省略形
fn build_user(email: String, username: String) -> User {
    User { // emailとusernameを取得して、インスタンスを返す 
        email, // email
        username, // username 
        active: true,
        sign_in_count: 1,
    }
}
```

## 構造体更新記法
他のインスタンスの情報を参考に、新たなインスタンスを生成する
```rust
let user2 = User {
    email: String::from("user2@example.com"),
    username: String::from("user2"),
    active: user1.active, // user1の情報を継承
    sign_in_count: user1.sign_in_count, // user1の情報を継承
};

let user3 = User {
    email: String::from("user3@example.com"),
    username: String::from("user3"),
    ..user1 // 残りの情報はuser1を継承
};
```

## タプル構造体
順番で引数が自明な場合は、タプル構造体を用いると良い
```rust
// タプル
let color: (i32, i32, i32) = (64, 64, 64);
let _a = color.0;

// タプル構造体
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
let white = Color(255, 255, 255);

struct Point(i32, i32, i32);
```
また関数の引数をColor型とすると、同じi32で構成されるPointであっても  型が異なるため、うまく動作しない。

## ユニット構造体
フィールドがない構造体で、主にポインタを引数に取る。詳細はChapter10で述べる

```rust
struct AlwaysEqual;
let subject = AlwaysEqual;
```

# リファクタリング
## タプルによるリファクタリング
以下の長方形の面積を求めるプログラムを考える
```rust
fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("{} pixcels", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```
上記の問題点として、関連する２つの引数（縦×横）がプログラム内で表現できていない

```rust
fn main() {
    let rect1 = (30, 50);
    println!("{} pixcels", area(rect1));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```
タプルによるリファクタリングで、引数が構造的になった。  
一方で、area()内の計算がタプルによる添字のアクセスとなったため、可読性が低下している

## 構造体によるリファクタリング
構造体を使用することで、添字を利用せず説明可能な構造・命名が可能となる
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{} pixcels", area(&rect1)); // 参照
}

fn area(rectagle: &Rectangle) -> u32 { //借用
    rectagle.width * rectagle.height
}
```

## デバッグ方法
println!()のマクロに対して、Displayの実装を追加する。  
（intなどには標準でDisplayが実装されている）  
そのため、構造体のデバッグには``println!({:?})``を指定子として使用し、構造体定義の直前で``derive(Debug)``の注釈を追加する。  
これによりprintln!()デバッグが可能となる。
```rust
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
```

