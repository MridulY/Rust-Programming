//#[derive(Debug)]
// enum IPadressKind{
//     V4,
//     V6,
// }

// struct IPaddress{
//     kind: IPadressKind,
//     address: String,
// }

// fn main(){
//     let four = IPadressKind::V4;
//     let six = IPadressKind::V6;

//     let localhost = IPaddress{
//         kind: IPadressKind::V4,
//         address: String::from("127.0.0.1");
//     }
// }

// fn main(){
//     let some_number: option<i32> = Some(5);
//     let some_string: option<String> = Some("Mridul");

//     let absent: option<i32> = None;
// }

// enum GenderCategory{
//     Male,
//     Female,
// }
// #[derive(Debug)]
// struct Employee{
//     name: String,
//     gender: GenderCategory
// }

// fn main(){
//     let x = GenderCategory::Male;
//     let y = GenderCategory::Female;
//     println!("x is {:?}", x);
//     println!("y is {:?}", y);

//     let a = Employee{
//         name: String::from("Mridul"),
//         gender: GenderCategory::Male,
//     };

//     println!("name of the employee is: {}", a.name);
//     println!("gender of the employee is: {:#?}", a);
// }
// #[derive(Debug)]
// enum IP{
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main(){
//     let home_IP = IP::V4(127, 0, 0, 1);
//     let office_IP = IP::V6(String::from("::1"));

//     println!("home ip address is: {:?}", home_IP);
// }

// fn divide(numerator: u32, denominator: u32) -> Option<u32>{
//     if denominator == 0{
//         None
//     }else{
//         Some(numerator/denominator)
//     }
// }

// fn main(){
//     let result1 = divide(20, 10);
//     let result2 = divide(30, 3);

//     match result1{
//         Some(value) => println!("result 1 is {}", value),
//         None => println!("result 1 is NULL"),
//     }

//     match result2{
//         Some(value) => println!("result 2 is {}", value),
//         None => println!("result 2 is NULL"),
//     }
// }

// enum Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value(x: Coin) -> u8{
//     match x{
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn main(){
//     println!("value of given is: {}",value(Coin::Dime));
// }

// fn main(){
//     let dice = 5;
//     match dice{
//         3 => dice_1(),
//         5 => dice_2(),
//         other => dice_3(other)
//     }
// }

// fn dice_1(){
//     println!("Hello");
// }

// fn dice_2(){
//     println!("Mridul");
// }

// fn dice_3(x: u8){
//     println!("value is matched to: {}", x);
// }

fn main(){
    let x = Some(5u8);
    if let Some(v) = x{
        println!("YOYOYO");
    }
}