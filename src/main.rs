use rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::stdin;

fn main() {
    
    let user1=User{
        name: String::from("Jone"),
        age: 31,
        step: 4,
        is_student: true
    };
    println!("{}", user1.find_age());
    // user1.age=32;
    // println!("about user: {user1:#?} ");
    // let user2=User{
    //     age: 50,
    //     ..user1
    // };
    // 
    // println!("{user2:#?}");
    
    // dbg!(&user1);
    // 
    // let mut s = String::from("hello");
    // find_first(&mut s[..3]);
    // find_first(&mut s[3..]);
    // summary_problems();
    // test_fn(47,'a');
    // start();
    
}
#[derive(Debug)]
struct User {
    name: String,
    age: i8,
    step: i8,
    is_student: bool,
    
}
impl User{

    fn find_age(&self)->i8{
        self.age*self.step
    }
}

fn find_first(s: &mut str){
    println!("{s}")
}

fn summary_problems(){
    let mut str=String::new();
    stdin()
        .read_line(&mut str)
        .expect("Failed read line.");
    let mut n: i32= match str.trim().parse(){
        Ok(num)=>num,
        Err(_)=>0
    };
    let mut fibo1 = 0;
    let mut fibo2 = 1;
    loop {
        if n==0 {
            println!("Finish.");
            break
        }
        println!("{n} : {fibo2}");
        fibo2=fibo1+fibo2;
        fibo1=fibo2-fibo1;
        n=n-1;
    }

}

fn test_fn(x: u32, c: char)-> u32{
    let arr = [1, 2, 3, 4, 5];
    let mut indx=0;
    while arr.len()-1>=indx {
        println!("result: {}", arr[indx]);
        indx=indx+1
    }

    for item in (1..6).rev() {
        println!("{item}")
    }
    let mut count=0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remain=0;
        loop {
            if count>1 {
                break 'counting_up;
            };
            if remain > 9 {
                break;
            }
            println!("remain: {remain}");
            remain = remain + 1;
        };
        count=count+1;
    }

    let res:String={
        let res="hello world";
        res.len().to_string()
    };

    if match res.trim().cmp("11"){
        Ordering::Equal=>true,
        Ordering::Greater=>false,
        Ordering::Less=> false
    }{
        println!("{res} is bigger then 5");
    }else {
        println!("{res} is not valid");
    }

    println!("{res}");

    let x=x+1;
    println!("{c} is equals to {x} in ASCII code.");
    x
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
