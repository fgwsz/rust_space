fn main() {
    let mut s=String::from("hello");//&str字符串字面量（不可变）来构建String mut（可变的）
    s.push_str(",world!");//push_str()在字符串后面追加字面值
    println!("{}",s);//hello,world!
    {
        let s=String::from("hello");//分配堆区内存allocate
    }//自动调用drop（类似c语言中的free,C++中的析构函数）释放堆区内存
    //上述实现就是C++中的RAII思想
    let x=5;//Rust中栈区的变量值（POD类型）传递都是值拷贝
    let y=x;//值拷贝
    println!("x:{x},y:{y}");//x:5,y:5

    let s1=String::from("hello");//注意：在Rust中String使用的RAII思路包了一个堆区的Char*
    let s2=s1;//这里不是值拷贝，而是移动，
    //与C++不同的是，Rust中变量的值被移动之后，
    //除非重新定义重名的变量覆盖，否则变量名都直接不能使用了。
    //println!("{},world!",s1);
    //|                    ^^ value borrowed here after move

    let s1=String::from("hello");
    let s2=s1.clone();//这里强制调用复制构造函数,拷贝一个副本赋值给s2
    //凡是涉及到堆区内存的变量，想要复制赋值就必须显式调用clone()
    println!("s1={},s2={}",s1,s2);//s1=hello,s2=hello
    //总结，栈上变量=是拷贝，堆上变量(或者使用了RAII内部使用堆区内存)=移动
    //堆上变量要复制只能=xxx.clone();
    {
        let s=String::from("hello");//s进入作用域
        takes_ownership(s);//hello//s的值被移动到函数里...
        let x=5;//x进入作用域
        makes_copy(x);//5//x应该移动到函数里
                      //但i32是Copy的，
                      //所以在后面可继续使用x
    }//这里，x先移出了作用域，然后是s。但因为s的值已被移走，
     //没有特殊之处

    {//在Rust中函数的返回值也会在返回的时候转移所有权
        let s1=gives_ownership();//gives_ownership()将返回值的所有权转移给s1
        let s2=String::from("hello");//s2进入作用域
        let s3=takes_and_gives_back(s2);//s2被移动到
                                        //takes_and_gives_back()中，
                                        //它也将返回值转移给s3
    }//这里，s3移出作用域并被丢弃。s2也移出作用域，但已被移走，
    //所以什么也不会发生。s1离开作用域并被丢弃

    let s1=String::from("hello");
    let (s2,len)=calculate_length(s1);//元组的结构化绑定
    println!("The length of '{}' is {}",s2,len);//The length of 'hello' is 5
}
fn takes_ownership(some_string:String){//some_string进入作用域
    println!("{}",some_string);
}//这里，some_string移出作用域并调用`drop`方法。
 //占用的内存被释放
fn makes_copy(some_integer:i32){//some_integer进入作用域
    println!("{}",some_integer);
}//这里，some_integer移出作用域。没有特殊之处。
fn gives_ownership()->String{//gives_ownership会将返回值移动给函数调用者
    let some_string=String::from("yours");//some_string进入作用域。
    some_string//返回some_string，并移出给函数调用者
}
//takes_and_gives_back将传入字符串并返回该值
fn takes_and_gives_back(a_string:String)->String{//a_string进入作用域
    a_string//返回a_string并移出给函数调用者
}
//这里我的一个疑问：如果函数返回值没有使用（接收者）怎么办？那么移出的值没有接收对象会直接`drop`吗？
//答案是对的，如果没有被其他对象接收，离开函数作用域就会被`drop`。

fn calculate_length(s:String)->(String,usize){//返回元组(String,usize)
    let length=s.len();//len()返回字符串的长度
    (s,length)//这里使用元组的目的是，归还String的所有权，并返回字符串的长度
}
