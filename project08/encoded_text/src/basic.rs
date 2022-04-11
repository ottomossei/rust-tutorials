pub fn main() {
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
}
