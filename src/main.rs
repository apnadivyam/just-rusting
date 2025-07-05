mod carol;
mod fibonacci;
mod game;
mod library;
mod my_utils;
mod sort;
mod temprature;
mod test;

fn main() {
    print!(
        "What do you want to do ?
    1. Test ground
    2. Guessing game
    3. Convert Tempratures
    4. Generate nth Fibonacci
    5. Christmas carol “The Twelve Days of Christmas”
    6. Library
    7. Sort an array \n"
    );
    match my_utils::input_num() {
        1 => test::test(),
        2 => game::guessing_game(),
        3 => temprature::converter(),
        4 => fibonacci::nth_fibonacci(),
        5 => carol::christmas(),
        6 => library::library(),
        7 => sort::sorting_an_array(),
        _ => println!("Bad Choice !!!"),
    }
}
