use std::io;

fn main() {
    let a=[1,2,3,4,5];
    let months=["January","February","March","April","May","June","July",
                "August","September","October","November","December"];
    let a:[i32;5]=[1,2,3,4,5];
    let a=[3;5];// <=> let a=[3,3,3,3,3];
    let first=a[0];
    let second=a[1];

    let a=[1,2,3,4,5];
    println!("Please enter an array index.");
    let mut index=String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index:usize=index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element=a[index];
    println!("The value of the element at index {index} is:{element}");
}
