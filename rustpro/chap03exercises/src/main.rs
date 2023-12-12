//相互转换摄氏与华氏温度
//解题思路：
//摄氏温度的英文写作Celsiustemperature，华氏温度的英文写作Fahrenheittemperature。
//将摄氏温度转换为华氏温度，使用以下公式：F=(C*9/5)+32，其中F为华氏温度，C为摄氏温度。
//将华氏温度转换为摄氏温度，使用以下公式：C=(F-32)*5/9，其中C为摄氏温度，F为华氏温度。
fn celsius_to_fahrenheit(temperature:f64)->f64{
    (temperature*9.0/5.0)+32.0
}
fn fahrenheit_to_celsius(temperature:f64)->f64{
    (temperature-32.0)*5.0/9.0
}
fn test01(){
    let celsius_temperature:f64=20.0;
    println!("The value of celsius_temperature is:{celsius_temperature}");
    let fahrenheit_temperature:f64=celsius_to_fahrenheit(celsius_temperature);
    println!("The value of fahrenheit_temperature:{fahrenheit_temperature}");
    let celsius_temperature:f64=fahrenheit_to_celsius(fahrenheit_temperature);
    println!("The value of celsius_temperature is:{celsius_temperature}");
    //The value of celsius_temperature is:20
    //The value of fahrenheit_temperature:68
    //The value of celsius_temperature is:20
}
//生成第n个斐波那契数
//解题思路：
//斐波那契数列是由两个初始值为0和1的数开始，后面的每个数都是前面两个数之和。
//递归函数的定义为：f(n)=f(n-1)+f(n-2)，其中f(0)=0，f(1)=1。
//递归实现
fn fibonacci_1(n:u64)->u64{
    if n<2{n}else{fibonacci_1(n-1)+fibonacci_1(n-2)}
}
//循环实现
fn fibonacci_2(n:u64)->u64{
    if n<2{
        return n;
    }
    let mut c_n=2;
    let mut fib_n_1=1;
    let mut fib_n_2=0;
    let mut temp=0;
    while c_n!=n{
        c_n+=1;
        temp=fib_n_1;
        fib_n_1=fib_n_1+fib_n_2;
        fib_n_2=temp;
    }
    fib_n_1+fib_n_2
}
fn test02(){
    let n:u64=20;
    println!("fibonacci_1({n})={}",fibonacci_1(n));
    println!("fibonacci_2({n})={}",fibonacci_2(n));
    //fibonacci_1(20)=6765
    //fibonacci_2(20)=6765
}
//打印圣诞颂歌"The Twelve Days of Christmas"的歌词，并利用歌曲中的重复部分（编写循环）
//On the first day of Christmas
//My true love gave to me
//A partridge in a pear tree
//
//On the second day of Christmas
//My true love gave to me
//Two turtle doves
//And a partridge in a pear tree
//
//On the third day of Christmas
//My true love gave to me
//Three French hens
//Two turtle doves
//And a partridge in a pear tree
//
//On the fourth day of Christmas
//My true love gave to me
//Four calling birds
//Three French hens
//Two turtle doves
//And a partridge in a pear tree
//
//On the fifth day of Christmas
//My true love gave to me
//Five gold rings
//Four calling birds
//Three French hens
//Two turtle doves
//And a partridge in a pear tree
//
//On the sixth day of Christmas
//My true love gave to me
//Six geese a-laying
//Five gold rings
//Four calling birds
//Three French hens
//Two turtle doves
//And a partridge in a pear tree
//
//On the seventh day of Christmas
//My true love gave to me
//Seven swans a-swimming
//Six geese a-laying
//Five gold rings
//Four calling birds
//Three French hens
//Two turtle doves
//And a partridge in a pear tree
//
//On the eighth day of Christmas
//My true love gave to me
//Eight maids a-milking
//Seven swans a-swimming
//Six geese a-laying
//Five gold rings
//Four calling birds
//Three French hens
//Two turtle doves
//And a partridge in a pear tree
//
//On the ninth day of Christmas
//My true love gave to me
//Nine ladies dancing
//Eight maids a-milking
//Seven swans a-swimming
//Six geese a-laying
//Five gold rings
//Four calling birds
//Three French hens
//Two turtle doves
//And a partridge in a pear tree
//
//On the tenth day of Christmas
//My true love gave to me
//Ten lords a-leaping
//Nine ladies dancing
//Eight maids a-milking
//Seven swans a-swimming
//Six geese a-laying
//Five gold rings
//Four calling birds
//Three French hens
//Two turtle doves
//And a partridge in a pear tree
//
//On the eleventh day of Christmas
//My true love gave to me
//Eleven pipers piping
//Ten lords a-leaping
//Nine ladies dancing
//Eight maids a-milking
//Seven swans a-swimming
//Six geese a-laying
//Five gold rings
//Four calling birds
//Three French hens
//Two turtle doves
//And a partridge in a pear tree
//
//On the twelfth day of Christmas
//My true love gave to me
//Twelve drummers drumming
//Eleven pipers piping
//Ten lords a-leaping
//Nine ladies dancing
//Eight maids a-milking
//Seven swans a-swimming
//Six geese a-laying
//Five gold rings
//Four calling birds
//Three French hens
//Two turtle doves
//And a partridge in a pear tree
//解题思路：
//建立一个数组然后逆序输出就可以了。
fn print_the_twelve_days_of_christmas(){
    const ARRAY_1:[&str;12]=[
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];
    const ARRAY_2:[&str;12]=[
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    let mut index:usize=0;
    let mut count:usize=0;
    for element in ARRAY_1{
        println!("On the {element} day of Christmas");
        println!("My true love gave to me");
        count=index;
        loop{
            println!("{}",ARRAY_2[count]);
            if count==0{
                break;
            }
            count-=1;
        }
        println!("");
        index+=1;
    }
}
fn test03(){
    print_the_twelve_days_of_christmas();
    //On the first day of Christmas
    //My true love gave to me
    //And a partridge in a pear tree
    //
    //On the second day of Christmas
    //My true love gave to me
    //Two turtle doves
    //And a partridge in a pear tree
    //
    //On the third day of Christmas
    //My true love gave to me
    //Three French hens
    //Two turtle doves
    //And a partridge in a pear tree
    //
    //On the fourth day of Christmas
    //My true love gave to me
    //Four calling birds
    //Three French hens
    //Two turtle doves
    //And a partridge in a pear tree
    //
    //On the fifth day of Christmas
    //My true love gave to me
    //Five gold rings
    //Four calling birds
    //Three French hens
    //Two turtle doves
    //And a partridge in a pear tree
    //
    //On the sixth day of Christmas
    //My true love gave to me
    //Six geese a-laying
    //Five gold rings
    //Four calling birds
    //Three French hens
    //Two turtle doves
    //And a partridge in a pear tree
    //
    //On the seventh day of Christmas
    //My true love gave to me
    //Seven swans a-swimming
    //Six geese a-laying
    //Five gold rings
    //Four calling birds
    //Three French hens
    //Two turtle doves
    //And a partridge in a pear tree
    //
    //On the eighth day of Christmas
    //My true love gave to me
    //Eight maids a-milking
    //Seven swans a-swimming
    //Six geese a-laying
    //Five gold rings
    //Four calling birds
    //Three French hens
    //Two turtle doves
    //And a partridge in a pear tree
    //
    //On the ninth day of Christmas
    //My true love gave to me
    //Nine ladies dancing
    //Eight maids a-milking
    //Seven swans a-swimming
    //Six geese a-laying
    //Five gold rings
    //Four calling birds
    //Three French hens
    //Two turtle doves
    //And a partridge in a pear tree
    //
    //On the tenth day of Christmas
    //My true love gave to me
    //Ten lords a-leaping
    //Nine ladies dancing
    //Eight maids a-milking
    //Seven swans a-swimming
    //Six geese a-laying
    //Five gold rings
    //Four calling birds
    //Three French hens
    //Two turtle doves
    //And a partridge in a pear tree
    //
    //On the eleventh day of Christmas
    //My true love gave to me
    //Eleven pipers piping
    //Ten lords a-leaping
    //Nine ladies dancing
    //Eight maids a-milking
    //Seven swans a-swimming
    //Six geese a-laying
    //Five gold rings
    //Four calling birds
    //Three French hens
    //Two turtle doves
    //And a partridge in a pear tree
    //
    //On the twelfth day of Christmas
    //My true love gave to me
    //Twelve drummers drumming
    //Eleven pipers piping
    //Ten lords a-leaping
    //Nine ladies dancing
    //Eight maids a-milking
    //Seven swans a-swimming
    //Six geese a-laying
    //Five gold rings
    //Four calling birds
    //Three French hens
    //Two turtle doves
    //And a partridge in a pear tree
    //
}
fn main() {
    test01();
    test02();
    test03();
}
