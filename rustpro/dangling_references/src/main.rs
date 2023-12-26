fn main() {
    let reference_to_nothing=dangle();
    println!("{}",reference_to_nothing); // hello
}
// fn dangle()->&String{
//     let s=String::from("hello");
//     &s // 返回一个局部变量(栈变量)的引用
// }
// error[E0106]: missing lifetime specifier
//  --> src\main.rs:4:14
//   |
// 4 | fn dangle()->&String{
//   |              ^ expected named lifetime parameter
//   |
//   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// help: consider using the `'static` lifetime
//   |
// 4 | fn dangle()->&'static String{
//   |               +++++++
fn dangle()->String{
    let s=String::from("hello");
    s // 返回一个局部变量(栈变量)的所有权，移动右值出去，就不涉及悬垂引用的问题了。
}
