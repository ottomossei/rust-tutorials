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
