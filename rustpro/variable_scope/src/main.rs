fn main() {
    {//s在这里无效，它尚未声明
        let s="hello";//从此处开始，s是有效的
        //使用s
    }//此作用域已结束，s不在有效
//    println!("The value of s is:{s}");
//|                                ^
//|
//help: the binding `s` is available in a different scope in the same function
}
