fn main() {
    let mut s=String::from("hello");//&str字符串字面量（不可变）来构建String mut（可变的）
    s.push_str(",world!");//push_str()在字符串后面追加字面值
    println!("{}",s);//hello,world!
    {
        let s=String::from("hello");//分配堆区内存allocate
    }//自动调用drop（类似c语言中的free,C++中的析构函数）释放堆区内存
    //上述实现就是C++中的RAII思想
}
