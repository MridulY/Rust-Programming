use std::io;

fn main(){
    let number = 8;
    println!("Enter the number you want to compare");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("failed");

    let x = x.trim().parse().expect("failed");
    
    if x < number{
        println!("less number");
    }else if number == x{
        println!("same number");
    }else{
        println!("high number");
    }
}

fn main(){
    let mridul = false;
    let x = if mridul {5} else{6};
    println!("value of x is: {x}");
}

fn main(){
    let mut x = 0;
    loop{
        println!("Mridul");
        x += 1;
        if x == 5{
            break;
        }
    }
}

fn main(){
    let mut number = 10;
    while number != 0{
        println!("{number}");
        number -= 1;
    }
}

fn main(){
    let a = [10,20,30,40,50];
    let mut index = 4;
    while index < 5{
        println!("value of index is: {}", a[index] );
        index += 1;
    }
}

fn main(){
    let a = [1,2,3,4,5,6,7,8,9,10];
    for ele in a{
        println!("value of element is: {ele}");
    }
}

fn main(){
    for ele in (1..5).rev(){
        println!("{ele}");
    }
}

fn main(){
    let n = 5;
    let mut x = 0;
    let mut y = 1;
    let mut sum = x+y;
    let mut cnt = 1;
    println!("value of nth fibonacci series is: {x}");
    println!("value of nth fibonacci series is: {y}");
    println!("value of nth fibonacci series is: {sum}");
    while cnt != n{
        x = y;
        y = sum;
        sum = x+y;
        cnt += 1;
        println!("value of nth fibonacci series is: {sum}");
    }
}