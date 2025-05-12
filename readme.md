jlox解释器的 Rust版本，
不追求效率,因为初学Rust，只能按Java的思路来写.

* 词法分析
   * 为了简单，解释器只接收文件作为输入 
* 错误处理
  理论上应该用Result<T,E>返回，但是为了简单，直接Panic或者expected
