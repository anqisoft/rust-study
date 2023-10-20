// error/004.rs
// https://www.runoob.com/rust/rust-basic-syntax.html

static a: i32 = 123;
a = "abc";
a = 4.56; 
a = 456;

/* result:
error: expected one of `!` or `::`, found `=`
 --> 003_error4.rs:4:3
  |
4 | a = "abc";
  |   ^ expected one of `!` or `::`

error: aborting due to previous error
*/
