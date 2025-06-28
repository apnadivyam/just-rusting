use crate::my_utils::{input_num, random_numb};

pub fn guessing_game() {
    println!("Hello, world, welcome to rust games !");
    println!(
        "Guess what's the secreat number in fewest attempts. Don't worry I'll count how many attempts you have made. Best on Luck 👍"
    );
    let target: i32 = random_numb();
    let mut trys: i32 = 1;
    let mut n: i32 = input_num();
    while n != target {
        if n < target {
            println!("Guess bigger");
        } else {
            println!("Guess smaller");
        }
        n = input_num();
        trys += 1;
    }
    println!("You Won 👏 after {} guesses", trys);
}
