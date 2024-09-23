use std::io::{self, Read};

fn main(){
    let mut choice = String::new();
    println!("What is your input? '1' for Fahrenheit, '2' for Santigrat");
    io::stdin().read_line(&mut choice).expect("Couldnt read line.");
    println!("{choice}");
    let choice: i32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => 5,
    };
    if choice == 1{
        let mut degree = String::new();
        io::stdin().read_line(&mut degree).expect("Couldnt read line.");
        let degree: f32 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => 5.0,
        };
        println!("Fahrenheit to Santigrat degree is: {}", f_to_c(degree));
    }
    if choice == 2{
        let mut degree = String::new();
        io::stdin().read_line(&mut degree).expect("Couldnt read line.");
        let degree: f32 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => 5.0,
        };
        println!("Santigrat to Fahrenheit degree is: {}", c_to_f(degree));
    }
}
fn f_to_c(x: f32) -> f32{
    (x-32.0)/1.8
}

fn c_to_f(x: f32) -> f32{
    (1.8 * x) + 32.0
}