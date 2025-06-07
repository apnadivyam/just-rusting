use rand::Rng;
use std::io;

pub fn guessing_game() {
    println!("Hello, world, welcome to rust games !");
    println!(
        "Guess what's the secreat number in fewest attempts. Don't worry I'll count how many attempts you have made. Best on Luck 👍"
    );
    let mut genrator: rand::prelude::ThreadRng = rand::thread_rng();
    let target: i32 = genrator.gen_range(0..100);
    let mut trys: i32 = 1;
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    let mut n: i32 = input.trim().parse().expect("can't parse this");
    input.clear();
    while n != target {
        if n < target {
            println!("Guess bigger");
        } else {
            println!("Guess smaller");
        }
        io::stdin().read_line(&mut input).expect("failed to read");
        n = input.trim().parse().expect("can't parse this");
        input.clear();
        trys += 1;
    }
    println!("You Won 👏 after {} guesses", trys);
}
