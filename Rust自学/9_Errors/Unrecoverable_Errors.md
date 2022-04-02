### Errors

---

+ Rust将error分成了`recoverable`和`unrecoverable`的errors。

+ 通常情况下，Rust用`panic!`这个macro来报告错误并退出。在这个过程中，rust会unwind并清理整个stack，之后推出。

  + 如果项目比较大的话，清理的过程是比较耗时的。我们也可以选择不清理，而是直接退出，让操作系统清理。这就需要在cargo.toml中加上：

  ```rust
  [profile.release]
  panic = 'abort'
  ```

+ 一个简单的例子：

  ```rust
  fn main() {
      panic!("crash and burn!");
  }
  ```

+ 其他panic的方式，比如数组越界：

  ```rust
  fn main() {
      let v = vec![1, 2, 3, 4];
      let a = &v[99];
  }
  ```

+ 也可以用`RUST_BACKTRACE=1`来backtrace一个错误。

  ```bash
  $ RUST_BACKTRACE=1 cargo run
  ```



