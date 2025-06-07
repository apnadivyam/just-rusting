use crate::my_utils::{self, BookStatus};

pub fn library() {
    let mut books = my_utils::book_factory();

    print!("Wellcome to the Grand Library. ");
    loop {
        print!(
            "What do you want to do ?
    1. Show all books
    2. Show Available books
    3. Borrow a book
    4. Return a book
    _. Exit Library\n"
        );
        match my_utils::input_num() {
            1 => {
                for i in 0..books.len() {
                    println!("{i}. {:#?}", books[i]);
                }
            }
            2 => {
                for i in 0..books.len() {
                    match books[i].status {
                        BookStatus::Available => println!("{i}. {:#?}", books[i]),
                        _ => {}
                    }
                }
            }
            3 => {
                println!("Enter the Book number: ");
                let bn = my_utils::input_num() as usize;
                if bn >= books.len() {
                    print!("{} is not a valid book number. ", bn);
                    continue;
                }
                match &books[bn].status {
                    BookStatus::Borrowed(name) => {
                        println!("This book is already issued to {}", name)
                    }
                    BookStatus::Lost => println!("This book has been reported Lost"),
                    BookStatus::Available => {
                        println!("Enter borrower's name here: ");
                        let name = my_utils::input_str().trim().to_string();
                        books[bn].status = BookStatus::Borrowed(name.clone());
                        println!("Book ISBN {} has been issued to {}", books[bn].isbn, name)
                    }
                }
            }
            4 => {
                println!("Enter the Book number: ");
                let bn = my_utils::input_num() as usize;
                if bn >= books.len() {
                    print!("Not a valid book number.");
                    continue;
                }
                match &books[bn].status {
                    BookStatus::Borrowed(name) => {
                        println!("Hi {}. Thanks for returning the Book.", name);
                        books[bn].status = BookStatus::Available;
                    }
                    BookStatus::Lost => println!("This book has been reported Lost"),
                    BookStatus::Available => {
                        println!("This book is already Available in the Library.")
                    }
                }
            }
            _ => return,
        }
    }
}
