# 1. パッケージ管理
用語
 - クレート
    - rustにおけるコンパイルの単位、木構造なモジュール群
 - パッケージ
    - クレートをビルドし、共有できるcargoの機能
 - モジュール
     - `use`を使用することで、パスの構成、スコープ、公開の可否を決定できる
 - パス
    - 構造体やモジュール自体に名前を付ける機能

## 1.1. パッケージとクレート
クレートはバイナリかライブラリのどちらかをとる。  
クレートルート（create root）はRustコンパイラの開始点であり、クレートのルートモジュールとなっている。  
パッケージは1つ以上のクレートで構成され、`Cargo.toml`で管理する。  

```bash
// プロジェクト作成
$ cargo new hello_cargo
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```
Cargo.tomlの中身は以下の通り
```toml
[package]
name = "hello_cargo" # プロジェクト名
version = "0.1.0"
edition = "2021"
[dependencies]
rand = "0.3.14" # randクレート（ライブラリクレート）
```
今、このパッケージには`src/main.rs`のみが含まれており、これはパッケージと同じ名前を持つバイナリクレートのクレートルートである。  
またrandクレートが提供する機能は全てクレート名`rand`を使用してアクセスするため、命名の衝突がなくクレート内外の名前空間が衝突することはない。

## 1.2. 新しいライブラリの作成
以下のコマンドで新しいライブラリを作成できる
```bash
$ cargo new --lib restaurant
```
このrestaurantディレクトリはライブラリ直下のディレクトリに置かれる。

## 1.3. モジュールシステム
モジュールは`mod {モジュール名}`で定義され、モジュール内にさらにモジュールも定義できる。  
仮に以下のコードを書いた場合、モジュールツリーは以下の通りである。
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```
モジュールツリー
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
親族関係の比喩を使って、モジュールAがモジュールBの中に入っている時、AはBの`子`であるといい、BはAの`親`であるという。  
またモジュールツリー全体が、暗黙のうちに作られたcrateというモジュールの下にある。

## 1.4. モジュールツリーの要素を示すためのパス
ファイルシステム同様に、関数を呼び出す場合はパスを利用する。
 - 絶対パス
    - クレート名、もしくは`create`という文字列より、クレートルートから開始する。
 - 相対パス
    - `self`、`super`、もしくは今のモジュール内の識別子を利用する。

ここで`add_to_waitlist`を呼ぶ場合は以下のように呼び出すことができる。  
また`pub`で公開可能な状態にしている。

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    // 相対パス
    front_of_house::hosting::add_to_waitlist();
}
```

相対パスを使うか絶対パスを使うかは、プロジェクトに依る。  
例えば、front_of_houseモジュールとeat_at_restaurant関数をcustomer_experienceというモジュールに移動させると、add_to_waitlistへの絶対パスを更新する必要があるが、相対パスは有効なままだ。  
しかし、eat_at_restaurant関数だけをdiningというモジュールに移動させると、add_to_waitlistへの絶対パスは同じままだが、相対パスは更新する必要がある。  
すなわちこの例では、`関数とモジュールの結びつきが強く、パス変更を同時に行なうことが多い`場合は相対パス、`関数とモジュールの結びつきが弱く、関数のみ他のモジュールに移動`する場合は絶対パスとなる。  

## 1.5. パスをpubキーワードで公開する
コンパイルを通すためには、使用する構造体、enum、メソッド全てを`pub`で公開する必要がある。
ここで`front_of_house`は公開されていないが、`eat_at_restaurant`と同じモジュール内で定義された兄弟であるため参照可能である。
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}
```

## 1.6. super
`super`を利用することで、上の階層の関数を参照することができる。
```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

## 1.7. 構造体の公開
構造体定義の前にpubを使うと、構造体自身は公開されるが、構造体のフィールドは非公開のままとなり、各フィールドを公開するか否かを個々で設定できる。
```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // Error : seasonal_fruitは非公開
    meal.seasonal_fruit = String::from("blueberries");
}
```

## 1.8. enumの公開
enumは構造体と異なり、各々で公開・非公開を設定できない

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // SoupもSaladも公開されている
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

