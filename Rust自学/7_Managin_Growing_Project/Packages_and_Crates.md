### Packages_and_Crates

---

+ Crate: 是一个二进制文件binary或者一个library。

+ `crate root`是rust compiler编译的起点。

+ 一个`package`中有一个或多个的`crates`，这些`crates`提供fucntionalities。在`package`中，由Cargo.toml来描述如何build这些crates

+ 我们以一个简单的`cargo new`为例：

  ```bash
  $ cargo new Packages_and_Crates
  ```

  + 它自动创好了下面的文件：

  ![image-20220327183150855](C:\Users\LENOVO\AppData\Roaming\Typora\typora-user-images\image-20220327183150855.png)

  + 看`Cargo.toml`文件：

  ```toml
  [package]
  name = "Packages_and_Crates"
  version = "0.1.0"
  edition = "2021"
  
  # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
  
  [dependencies]
  ```

  + 里面并没有包含`src/main.rs`。因为，按照rust的传统，`src/main.rs`就是`crate root` of binary crate。
  + 同理，对于library crate，它的`root crate`默认就是`src/lib.rs`
    + 这里，我们的package中只有`src/main.rs`, 说明这个package中只包含一个crate

+ 一个crate往往包含的是相近功能的代码，比如在第二章中用到的`rand`。 我们当时是把rand写到Cargo.toml的[dependencies]下，从而将这个crate引入