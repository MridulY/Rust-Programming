// fn main(){
//     let s = vec![1,2,3,4];
//     println!("{:?}", s);
//     ve(&s);
//     // let x = s.clone();
//     println!("{:?}", s);
//     // let mut x = String::from("hello");
//     // x.push_str(" Mridul");
//     // println!("{:?}", x);

//     // let a = 5;
//     // let b = a;
//     // println!("x = {}, y = {}", a, b);
//     let b = s.len();
//     println!("size of vector is {b}");
// }

// fn ve(y:&Vec<i32>){
//     println!("{:?}", y);
// }

fn main(){
    let s = String::from("Hello World");
    let a = first_word(&s);
    println!("{}", a);
}

fn first_word(u: &String) -> &str{
    let bytes = u.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item ==b' '{
            return &u[0..i];
        }
    }
    &u[..]
}

// fn main(){
//     let s = String::from("Hello");
//     let len = s.len();
//     // println!("{}",s);
//     // s.clear();
//     // println!("{}",s);
//     let slice = &s[..3];
//     println!("{}", slice)

// }