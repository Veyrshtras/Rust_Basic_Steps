use rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let res:usize={
        let res="salom";
        res.len()
    };
    
    println!("{res}");
    test_fn(47,'a');
    start(); 
}
fn test_fn(x: u32, c: char)-> u32{
    println!("{c} is equals to {x} in ASCII code.");
    return x
}
fn start(){
    let arr=[2;7];
    println!("{}",arr[3]);

    let tuple:(u32, f64, char)=(12, 45.78, 'a');
    println!("result: {}",tuple.1);
    const X: i32=5;
    println!("result: {X}");
    // x=7;
    // println!("result: {x}");
    println!("guess the number!");
    loop {
        println!("enter guessed number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read file.");
        println!("you guessed: {}", guess);
        let number = rand::rng().random_range(1..=100);
        println!("result: {number}");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&number) {
            Ordering::Less => println!("low then number"),
            Ordering::Greater => println!("greater then number"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}