use std::io;0.


fn main(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let first = tup.0;
    let second = tup.1;
    let third = tup.2;

    println!("value of first is {first}");
    println!("value of second is {second}");
    println!("value of third is {third}")

    let a = [1,2,3,4,5];
    println!("value at second index is: {}", a[2]);

    println!("Enter the index you want to access");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Wrong input");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index is not a number");

    let element = a[index];

    println!("Number at index is: {element}");
}