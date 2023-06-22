//this programs counts down from the end range to the start range provided by the user

use std::io;
fn main (){
    let mut  start=String::new();
    let mut  end=String::new();
    println!("beginner rust program... countdown");
    println!("enter the start range");
    io::stdin().read_line(&mut start).expect("failed to read first number!");
    println!("enter the end range");
    io::stdin().read_line(&mut end).expect("failed to read second number!");
let start:u32=start.trim().parse().expect("failed ");
let end:u32=end.trim().parse().expect("failed");
    for number in (start..end).rev(){
        println!("{}",number);

    }
    println!("Lift off");
}