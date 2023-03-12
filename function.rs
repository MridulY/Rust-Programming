fn main(){
    println!("Calling game function");

    //another_function(5);
    game(70, 'A');
    let y = {
        let x = 5;
        x+2
        //println!("value of x is {x}")
    };
    println!("value of x is: {y}");
    let z = five();
    println!("value of z is: {z}");
}

fn another_function(x: i32){
    println!("value of x is: {x}");
}

fn game(health: u64, level: char){
    println!("status of the her0 is: {health} {level}")
}

fn five() -> i32 {
    5
}