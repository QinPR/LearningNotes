### If_Let

---

+ 有时候我们用match的话是十分verbose的，因此，我们可以选择用`if let`的方式来进行判断。比如对于下面的match：

  ```rust
  fn main() {
      let config_max = Some(3u8);
      match config_max {
          Some(max) => println!("The maximun is configured to be {}", max),
          _ => (),
      }
  }
  ```

+ 我们可以将其用`if let`简化为：

  ```rust
  if let Some(max) = config_max{
      println!("The maximum is configured to be {}", max);
  }
  ```

  + 这里我们可以理解if let为**将match的一个pattern单独提取了出来**。在上面的例子中Some(max)就相当于是一个pattern，config_max match了这个pattern变回执行下面的代码。如果没有match的话这个`if let`语句就不会被执行。
  + 虽然这样就失去了match的exhaustive的性质。所以用match还是if let是一个trade off

+ 我们也可以用`else`来处理剩余的情况，但这样config_max的值是不会绑定的（我们没法用config_max的值）。

  ```rust
  if let Some(max) = config_max{
      println!("The maximum is configured to be {}", max);
  }else{
      println!("it is None");
  }
  ```

  

