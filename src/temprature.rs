use std::io;

pub fn converter() {
    println!("Enter the Temprature : ");
    let mut temp: String = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Cant read temprature.");
    let temp: f32 = temp.trim().parse::<f32>().expect("cant parse temp");
    println!("--- 1. Celcius to Fahrenheit\n--- 2. Fahrenheit to Celcius");
    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("cont read choice");
    let choice = choice.trim();
    if choice == "1" {
        println!("Fahrenheit => {}", temp * 1.8 + 32.0);
    } else if choice == "2" {
        println!("Celcius => {}", (temp - 32.0) * (5.0 / 9.0));
    } else {
        println!("Bad Choice ");
    }
}
