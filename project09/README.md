# エラー処理
## panic!処理
意図的にパニック処理を実施することができる。  
```rust
panic!("crash and burn");
```

また`panic!バックトレース`を使用することで、エラーが発生するまでに呼び出された全関数がわかる。  
```rust
fn main() {
    let v = vec![1, 2, 3];
    // Error : バッファー外読み出し
    v[99];
}
```
```shell
$ RUST_BACKTRACE~1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /checkout/src/liballoc/vec.rs:1555:10
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:381
   3: ...
```
## Results<T, E>の中身の調べ方
Result型はOkとErrの2列挙子から定義されており、Results<T,E>である。
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
具体的なResult<T,E>の中身に関しては、エラーを見ることでわかる。  
例えば、File::openはu32を返さないことは自明なので、あえてエラーを実行すると以下のエラー文が得られる。
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```
```shell
error[E0308]: mismatched types
(エラー: 型が合いません)
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
`std::result::Result`
  |
  = note: expected type `u32`
  (注釈: 予期した型は`u32`です)
             found type `std::result::Result<std::fs::File, std::io::Error>`
  (実際の型は`std::result::Result<std::fs::File, std::io::Error>`です)
```

これにより、Result<T,E>はResult<std::fs::File, std::io::Error>であることがわかる。  

## マッチガードによるエラー処理
上記のpanic, Result, matchを使用したエラー処理は下記のように書ける。  
想定されるエラーは`File::open`の`io::Error`の`ErrorKind`を調べる。  
https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html  
ここでは、`ErrorKind::Notfound`を例として使用する。  
エラー分岐として、`if error.kind() == ErrorKind::Notfound`という条件式（マッチガード）を利用し、エラーハンドリングできる。  
また`ref`を追記することで、errorがマッチガード式にmoveされず、単に参照するために使用されている。  

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        // Openするファイルが見つからない場合のアーム
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                // Openできず、かつ新規ファイルも作成できない場合のアーム
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        // Openするファイルが見つかったが、権限がない等でOpenできない場合のアーム
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
}
```

## unwrapとexpect
Result<T, E>は、色々な作業をするヘルパーメソッドが多く定義されている。  
unwrapはResult値がOk列挙子ならOkの中身を返し、Err列挙子ならpanic!マクロを呼ぶ。  
これによりmatchを利用したpanic処理は短縮できる。
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```
またexpectを利用することで、panic!のエラーメッセージを自由に記載できる。  
unwrapのpanic!呼び出しは全て同じエラーメッセージを出力するため、expectの方がエラー元を特定しやすい。
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

## エラー委譲
関数内でpanic!呼び出しせず、エラーを委譲することで、呼び出し元がエラーをどうするか決定させることもできる。
```rust
use std::io;
use std::io::Read;
use std::fs::File;

// 関数の戻り型がResult型
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    // Openする
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // エラー時は早期にreturnする
    };

    let mut s = String::new();

    // ファイルの中身をsに書き込む
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e), // 関数最後の式なので、returnは不要
    }
}
```

## ?演算子
`?`演算子を利用することで、上記と同様のコードが以下のように書ける。
```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
?はResult値がOkなら処理継続、Errorならreturnする。  
より厳密には、?は標準ライブラリの`Fromトレイト`で定義されており、エラーの型を別のものに変換する`from関数`が使用されている。  
よりエルゴノミックな書き方も下記のようにできる。
```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```
また?演算子はResultを返す関数でのみ使用できるため、main()の中等では利用できないので注意が必要だ。
