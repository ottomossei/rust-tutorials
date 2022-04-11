# 1. コレクション
コレクションはタプル型と異なり、データがヒープ上に確保されるため、データ量がコンパイル時には不明でも良い。したがって、伸縮可能である。
 - ベクタ型
   - 可変長の値を並べて保持出来る
 - 文字列
   - String型など、文字のコレクション
 - ハッシュマップ
   - 値とキーが紐づけされている。

## 1.1. ベクタ
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

## 1.2. Enum利用による複数の型の保持
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

## 1.3. よくあるエラー
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

## 1.4. 文字列
文字列の生成及び更新は以下の通りである。
```rust
// 新たな空の文字列
// let mut s = String::new();

// String::from関数
let _s0 = String::from("initial contents");
println!("{}", _s0); // initial contents

// to_stringメソッドより、文字列リテラルからStringを生成
let data = "initial contents";
let _s1 = data.to_string();
println!("{}", _s1); // initial contents

let mut _s2 = String::from("foo");
_s2.push_str("bar");
_s2.pop();
_s2.push('c');
println!("{}", _s2); // foobac
```
なおpush_str()は引数の所有権の奪取を行わない。
```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2); // s2の所有権をpush_strは奪っていない
println!("s2 is {}", s2); // 上記より、s2を実行できる
```

## 1.5. ＋演算子による連結
+演算子はaddメソッドのString値呼び出し時（`fn add(self, s: &str) -> String {`）を使用している。  
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1はmoveされ、もう使用できないことに注意
println!("{}", s3)
```
`s3 = s1 + &s2;`は両文字列をコピーするように見えるが、実際はs1の所有権を奪い、s2の中身のコピーを追記し、結果の所有権を返している。  
ここで、上記のadd関数に関するポイントを述べる。
 - s1が所有権を失う理由
   - selfには&がないため、s1はadd呼び出しにmoveされ、その後の所有権が無効となる。 
 - &s2(&String)がaddの引数(&str)で利用できる理由
   - コンパイラが`参照外し型強制`より、&s2を&s2[..]に型強制する。
   - addがs引数の所有権を奪わないため、この処理後もs2が有効なStringとなる。

## 1.6. format!マクロによる連結
複数の文字列を同時に連結する場合は、format!マクロが使いやすい
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

// s1, s2, s3の所有権を奪わない
let s = format!("{}-{}-{}", s1, s2, s3); // tic-tac-toe
```

## 1.7. 文字列中の各文字にアクセスする
以下はエラーとなる。
```rust
let s1 = String::from("hello");
let h = s1[0]; // error
```
Rustの文字列は添字アクセスをサポートしていない。  
その理由は多言語対応により、各文字列のバイト数が一定でないためである。
```rust
// 4バイト
let len = String::from("Hola").len();]
// 12バイトではなく24バイト。各Unicodeスカラー値が2バイトであるため。
let len = String::from("Здравствуйте").len();
```

そのため、文字列スライス時も各文字列のバイトを意識する必要がある。
```rust
let hello = "Здравствуйте";
let s = &hello[0..4]; // Зд
// Error
// thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`'
// ('main'スレッドは「バイト添え字1は文字の境界ではありません; `Здравствуйте`の'З'(バイト番号0から2)の中です」でパニックしました)
let s = &hello[0..1];
```

個々のUnicodeスカラー値に対して処理を行う場合、char型を利用すると良い
```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
// न
// म
// स
// ्
// त
// े
```

## 1.8. ハッシュマップ
`HashMap<K, V>`は、 K型のキーとV型の値の対応関係を保持する。  
ベクタと全く同様に、ハッシュマップはデータをヒープに保持する。  
このHashMapはキーがString型、 値はi32型であり、キーは全て同じ型でなければならず、 値も全て同じ型でなければならない。
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// 同様に以下でも作成可能
let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

// アンダースコアにより、コンパイラはベクタのデータ型に基づきハッシュマップの型を推論する
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

## 1.9. ハッシュマップの所有権
Stringのような所有権のある値は、moveされハッシュマップが所有者となる。
```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_nameとfield_valueはmoveされ、ハッシュマップが所有権を持つ
```

## 1.10. ハッシュマップの値アクセスと更新
getメソッドやforループの走査より、ハッシュマップから値を取り出すことができる。  
また同名のキーをinsertすることで値を更新できる。  
entryメソッドよりEntryと呼ばれるEnumを返し、キーの存在を確認できる。
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
// getメソッド
let score = scores.get(&team_name);

// forループによるキーとバリューの走査
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// 値の上書き
scores.insert(String::from("Blue"), 25);

// キーがない場合のみ挿入（既にBlueは存在するため、insertされない）
scores.entry(String::from("Blue")).or_insert(50);
```

## 1.11. 古い値に基づいた更新
キーの値を探し、古い値に基づいてそれを更新することもできる。
```rust
use std::collections::HashMap;

let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    // or_insert関数は可変参照(&mut V)を返す
    let count = map.entry(word).or_insert(0);
    // count変数が持つ可変参照を*で参照を外す
    *count += 1;
}

// {"world": 2, "hello": 1, "wonderful": 1}
println!("{:?}", map);
```

## 1.12. ハッシュ関数
標準では、HashMapはサービス拒否(DoS)アタックに対して抵抗を示す暗号学的に安全なハッシュ関数を使用している。  
これは利用可能な最速のハッシュアルゴリズムではないが、パフォーマンスの欠落と引き換えに安全性を得ている。  
標準のハッシュ関数は遅すぎる場合、 異なるhasherを指定することで別の関数に切り替えることができる。
