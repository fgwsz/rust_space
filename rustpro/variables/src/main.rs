fn main() {
    // const variable or mutable variable
    let mut x=5; //let x=5;
    println!("The value of x is:{x}");
    x=6;
    println!("The value of x is:{x}");
    // constant
    const THREE_HOURS_IN_SECONDS:u32=60*60*3;
    // variable shadowing
    let x=5;
    let x=x+1;
    {
        let x=x*2;
        println!("The value of x in the inner scope is:{x}");
    }
    println!("The value of x is:{x}");
    // variable name shadowing with mutable variable
    let spaces="    ";
    let spaces=spaces.len();
    let mut spaces="    ";
    // spaces=spaces.len(); // error
    //|            ^^^^^^^^^^^^ expected `&str`, found `usize`
}
