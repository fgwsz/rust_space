fn main() {
    //loop{
    //    println!("again!");
    //}// enter ctrl-c to break

    let mut counter=0;
    let result=loop{//like a lambda function
        counter+=1;
        if counter==10{
            break counter*2;// break loop and return counter*2;
        }
    };
    println!("The value of result is:{result}");//The value of result is:20

    let mut count=0;
    'counting_up:loop{//具名循环
        println!("count={count}");
        let mut remaining=10;
        loop{
            println!("remaining={remaining}");
            if remaining==9{
                break;//只能跳出一层循环
            }
            if count==2{
                break 'counting_up;//可以跳出具名的循环
            }
            remaining-=1;
        }
        count+=1;
    }
    println!("End count={count}");
    //count=0
    //remaining=10
    //remaining=9
    //count=1
    //remaining=10
    //remaining=9
    //count=2
    //remaining=10
    //End count=2

    let mut number=3;
    while number!=0{
        println!("{number}!");
        number-=1;
    }
    println!("LIFTOFF!!!");
    //3!
    //2!
    //1!
    //LIFEOFF!!!

    let a=[10,20,30,40,50];
    let mut index=0;
    while index<5{
        println!("The value is:{}",a[index]);
        index+=1;
    }
    //The value is:10
    //The value is:20
    //The value is:30
    //The value is:40
    //The value is:50
    for element in a{
        println!("the value is:{element}");
    }
    //the value is:10
    //the value is:20
    //the value is:30
    //the value is:40
    //the value is:50
    for number in (1..4).rev(){//(1..4) is range[1,4),rev is reverse
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    //3!
    //2!
    //1!
    //LIFTOFF!!!
}
