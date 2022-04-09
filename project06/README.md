# 1. Enum(列挙型)
取りうる値を列挙し型とする。  
RustのenumはF#, Ocaml, Haskellなど関数型言語に存在する``代数的データ型``に最も似ている。

## 1.1. Enumの定義
IPアドレスは必ずv4, v6のどちらかであり、同時に双方になりえない。この場合、型としてenum(列挙型)で表現する。
```rust
fn main() {
    // 宣言
    enum IpAddrKind {
        V4,
        V6,
    }

    // Route関数
    fn route(ip_type:IpAddrKind){ //V4, V6双方に対応した型
        // process
    }

    // インスタンス生成
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
}
```

Enumの値はフィールドでも利用できる。
```rust
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind, // enum
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4, // enum
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6, // enum
        address: String::from("::1"),
    };
}
```

enumの各列挙子にデータを直接添付することで、上記と同様の処理ができる。
`構造体`と異なり、別の型も定義できる。
```rust
fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```

下記コードのように、変数の中で役割を持たすこともでき、構造体よりシンプルな定義ができる。
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 構造体の場合は、以下のように記載
struct QuitMessage; // ユニット構造体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // タプル構造体
struct ChangeColorMessage(i32, i32, i32); // タプル構造体
```

## 1.2. Enumにメゾットを定義
Enumは構造体同様、メソッドを定義できる。
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// implによるメソッド定義
impl Message {
    fn call(&self) {
        // method body would be defined here
        // メソッド本体はここに定義される
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## 1.3. Options
`Rustにはnullがない`代わりに、標準ライブラリにてOptionsが以下のように定義されている。  
Option\<T>型は 取得できないかもしれない値を表現する列挙型だ。  
ここで、SomeではなくNoneを使用する場合、コンパイラにOption<T>の型が何になるかを教えなければならない。  
None値だけでは、Some列挙子が保持する型をコンパイラが推論できないからである。
```rust
// 標準ライブラリの定義
enum Option<T> { // <T>はジェネリック型引数(project10)
    Some(T),
    None,
}

// 以下のように実装できる
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None; // Noneであってもi32の型を指定
```

ここで、下記の実装はエラーとなる。
```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

// no implementation for `i8 + std::option::Option<i8>`
let sum = x + y;
```
上記のエラーから、RustはT型の処理を行う前には、Option<T>をTに変換する必要がある。  
その処理を制御するのが`match`である。

## 1.4. match制御フロー演算子
`match`は非常に強力な制御フロー演算子であり、リテラル値、変数名、ワイルドカードやその他多数のもので制御することが可能である。
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        // 下記に4つの分岐（4つのアーム）
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

またenumにenumを付随させることで、より詳細なmatchもできる。

```rust
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

let pcoin = Coin::Penny;
let qcoin = Coin::Quarter(UsState::Alabama);
value_in_cents(pcoin); // Lucky penny!
value_in_cents(qcoin); // State quarter from Alabama!
```

## Option\<T>とmatch
Optionとmatchから動作を制御できる。
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

## matchは包括的
以下のコードは包括的でないため、エラーとなる
```rust
// error[E0004]: non-exhaustive patterns: `None` not covered
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

u8のような場合、`_`を使用することで`包括的`にカバーできる。
```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

## if let
matchは包括的であるため、1つのパターンのみmatchさせたい場合でも`_=>()`が必要であり冗長である。  
その場合は`if let`を使用する。
```rust
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
else
    println!("Others")
}
```


## まとめ
enumは列挙された値のどれかになりうる独自の型を生成する。  
`Option\<T>`によるエラー回避、enumの値にデータを含む場合は`match`や`if let`が使用できる。