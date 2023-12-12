fn main() {
    let y=6;
    //let x=(let y=6);//error,使用let的是一个语句，而不是一个表达式
    //let x=let y=6;//error
    let y={
        let x=3;//语句是没有返回值的
        x+1//注意这里没有;表示这是一个表达式，而不是语句
    };
    println!("The value of y is:{y}");//The value of y is:4
}
