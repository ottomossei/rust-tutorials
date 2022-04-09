# コレクション
コレクションはタプル型と異なり、データがヒープ上に確保されるため、データ量がコンパイル時には不明でも良い。したがって、伸縮可能である。
 - ベクタ型
   - 可変長の値を並べて保持出来る
 - 文字列
   - String型など、文字のコレクション
 - ハッシュマップ
   - 値とキーが紐づけされている。

## ベクタ
ベクタは単体のデータ構造でありながら複数の値を保持でき、それらの値をメモリ上に隣り合わせに並べる。  
ベクタには同じ型の値しか保持できない。  
```rust
// 空のベクタ生成
// Vec::new関数よりi32の型注釈を付随させ、i32の要素を保持するとコンパイラに指示
let v1: Vec<i32> = Vec::new();

// マクロ「vec!」より値が設定された新しいベクタの生成
let v2 = vec![1, 2, 3];

// ベクタの更新
v2.push(4);

// ドロップ条件
{
    let v = vec![1, 2, 3, 4, 5];
} // vはドロップ
```

またベクタの要素を読むには、添字記法（`&v[i]`）もしくはgetメソッド（`v.get(i)`）を使用する。
```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}

// 参照はpanicとなる(100番目はないため)
let not_exist = &v[100]; 
// getメソッドはNoneを返す
v.get(100);
```

forを利用することで、1要素だけでなく全要素を走査することができる
```rust
let v1 = vec![100, 32, 57];

// 各要素の表示
for i in &v1 {
    println!("{}", i);
}

let v2 = vec![100, 32, 57];

// 各要素に50を足す
// 可変参照の値を変更するためには、参照外し演算子（*）を使用する必要がある。
// （詳細はproject15）
for i in &mut v2 {
    *i += 50;
}
```

## Enum利用による複数の型の保持
enumを保持するベクタを作成することで、ベクタでも結果的に異なる型を保持できる。
しかしプログラム記載時点で、プログラムが実行時に取得し、ベクタに格納し得る全ての型を網羅できない場合には、このenum使用ができない。  
その場合、トレイトオブジェクトを使用する（project17）
```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

## よくあるエラー
可変借用と不変借用は同時に存在できないため、以下のコードはエラーとなる。  
新たな要素をベクタの終端に追加するとき、現在のベクタにメモリスペースがなければ、新しいメモリを割り当て、古い要素を新しいスペースにコピーする必要がある。  
その場合、最初の要素を指す参照は、解放されたメモリを指すことになるため、エラーを通知する。
```rust
let mut v = vec![1, 2, 3, 4, 5];
let first = &v[0]; // Error : immutable borrow occurs here
v.push(6); // Error : mutable borrow occurs here

// error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
println!("The first element is: {}", first);
```









