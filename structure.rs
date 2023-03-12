#[derive(Debug)]
struct Hero{
    name:String,
    health:i32
}

struct User{
    status: bool,
    username: String,
    email: String,
    sign_in_count: i32
}

fn _input_struct(mut x:Hero){
    x.name = String::from("PJ");
    x.health = 50;
    println!("Name of Hero is {}, health of Hero is {}", x.name, x.health);
}

fn main(){
    let mut hero_1 = Hero{
        name:String::from("Mridul"),
        health:70
    };
    println!("Name of Hero is {}, health of Hero is {}", hero_1.name, hero_1.health);

    hero_1.name = String::from("Pranjali Priya");
    hero_1.health = 24;

    println!("Name of Hero is {}, health of Hero is {}", hero_1.name, hero_1.health);

    input_struct(hero_

    let mut user1 = User{
        status: true,
        username: String::from("Mridul"),
        email: String::from("mridulyadav3173@gmail.com"),
        sign_in_count: 1,
    };

    let mut user2 = User{
        email: String::from("raosahab3173@gmail.com"),
        ..user1
    };

    println!("staus of user is: {}", user2.status);
    println!("username of user is: {}", user2.username);
    println!("email of user is: {}", user2.email);
    println!("sign in count of user is: {}", user2.sign_in_count);
    user1.email = String::from("monuyadav3173@gmail.com");
    println!("email is {}", user1.email);
}


//Tuple struct
struct color(i32, i32, i32);
struct point(i32, i32, i32);

fn main(){
    let green = color(1,5,7);
    let origin = point(1,1,1);
}

//unit-like struct
struct Mridul;

fn main(){
    let x = Mridul;
}

fn main(){
    let width = 30;
    let length = 10;

    println!("The area of the rectange is {}", area(width, length))}

fn area(width:u32, length:u32) -> u32{
    width*length
}

//ABOVE FUNCTION USING TUPLE
fn main(){
    let rectangle = (30,30);
    println!("The area of the rectange is {}", area(rectangle));
}

fn area(dimensions: (u32,u32)) -> u32{
    dimensions.0 * dimensions.1
}

//SAME CODE USING STRUCT
struct Rectangle{
    width: u32,
    length: u32
}

fn main(){
    let rect = Rectangle{
        width: 30,
        length: 30
    };

    println!("area of rectangle is: {}", area(&rect));
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.length
}
fn main(){
    let scale = 2;
    let rect = Rectangle{
        width: dbg!(30*scale),
        length: 40
    };

    //println!("rect is {:#?}", rect);
    dbg!(rect);
}

//METHOD SYNTAX
struct Rectangle{
    width: u32,
    length: u32
}

fn main(){
    let rect = Rectangle{
        width: 30,
        length: 30
    };
    println!("Rectangle is a square: {}", rect.isSquare());
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.length
    }
    fn isSquare(&self) -> bool{
        self.width == self.length
    }
}

fn main(){
    let rect1 = Rectangle{
        width: 30,
        length: 40
    };

    let rect2 = Rectangle{
        width: 10,
        length: 20
    };

    let rect3 = Rectangle{
        width: 15,
        length: 25
    };

    println!("can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3: {}", rect1.can_hold(&rect3));
}

impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.length > other.length
    }
}