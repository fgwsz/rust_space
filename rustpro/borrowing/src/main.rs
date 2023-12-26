fn main() {
    //let s=String::from("hello");
    let mut s=String::from("hello");
    //change(&s);
    change(&mut s);
    println!("{s}"); // hello,world

    let mut s=String::from("hello");
    let r1=&mut s;
    // let r2=&mut s; // 同一个时刻一个变量只能有一个可变引用
    // error[E0499]: cannot borrow `s` as mutable more than once at a time
    //   --> src\main.rs:10:12
    //    |
    // 9  |     let r1=&mut s;
    //    |            ------ first mutable borrow occurs here
    // 10 |     let r2=&mut s;
    //    |            ^^^^^^ second mutable borrow occurs here
    // 11 |     println!("{},{}",r1,r2);
    //    |                      -- first borrow later used here
    // println!("{},{}",r1,r2);

    let mut s=String::from("hello");
    {
        let r1=&mut s;
    }// r1离开了作用域，所以我们完全可以创建一个新的可变引用
    let r2=&mut s;

    let mut s=String::from("hello");
    let r1=&s; // 没问题
    let r2=&s; // 没问题，rust允许对一个变量拥有多个不可变引用
    // let r3=&mut s; // 大问题
    // rust规定如果一个变量不能同时拥有可变引用和不可变引用（引用的作用域重叠的时候）
    // println!("{},{},{}",r1,r2,r3);
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    //   --> src\main.rs:31:12
    //    |
    // 29 |     let r1=&s;
    //    |            -- immutable borrow occurs here
    // 30 |     let r2=&s;
    // 31 |     let r3=&mut s;
    //    |            ^^^^^^ mutable borrow occurs here
    // 32 |     
    // 33 |     println!("{},{},{}",r1,r2,r3);
    //    |                         -- immutable borrow later used here

    let mut s=String::from("hello");
    let r1=&s;// 没问题            --|r1 begin
    let r2=&s;// 没问题            --|r2 begin
    println!("{} and {}",r1,r2); //--|r1 end/r2 end
    //hello and hello
    // 此位置之后r1和r2不再使用
    let r3=&mut s;//没问题
    println!("{}",r3);
    //hello

    // 总结:rust中的引用变量的作用域不只是只有在{}包体的末尾失效，
    // 还在重新定义变量名/最后一次使用的地方失效。
    // 这是和C++中引用的变量生命周期最不一样的一点。
}
//fn change(some_string:&String){
fn change(some_string:&mut String){
    some_string.push_str(",world");
    //error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    // --> src\main.rs:6:5
    //  |
    //6 |     some_string.push_str(",world");
    //  |     ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    //  |
    //help: consider changing this to be a mutable reference
    //  |
    //5 | fn change(some_string:&mut String){
    //  |                        +++
}
