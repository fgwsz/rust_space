fn main() {
    let s1=String::from("hello");
    let len=calculate_length(&s1);
    println!("The length of '{}' is {}.",s1,len);//The length of 'hello' is 5.
}
fn calculate_length(s:&String)->usize{
    s.len()
}
