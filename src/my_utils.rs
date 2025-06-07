use std::fmt;
use std::io;

pub struct Book {
    pub isbn: String,
    author: String,
    price: i32,
    pub status: BookStatus,
    title: String,
}

pub enum BookStatus {
    Borrowed(String),
    Lost,
    Available,
}

// Manual implementation of Debug for BookStatus  ai genrated
impl fmt::Debug for BookStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BookStatus::Borrowed(name) => write!(f, "Borrowed({})", name),
            BookStatus::Lost => write!(f, "Lost"),
            BookStatus::Available => write!(f, "Available"),
        }
    }
}

// Manual implementation of Debug for Book  ai genrated
impl fmt::Debug for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Book")
            .field("isbn", &self.isbn)
            .field("author", &self.author)
            .field("price", &self.price)
            .field("status", &self.status)
            .field("title", &self.title)
            .finish()
    }
}

pub fn book_factory() -> Vec<Book> {
    let book1 = Book {
        isbn: "978-3-16-148410-0".to_string(),
        author: "Jane Austen".to_string(),
        price: 1500,
        status: BookStatus::Available,
        title: "Pride and Prejudice".to_string(),
    };

    let book2 = Book {
        isbn: "978-0-7432-7356-5".to_string(),
        author: "Dan Brown".to_string(),
        price: 2000,
        status: BookStatus::Borrowed("Alice Johnson".to_string()),
        title: "The Da Vinci Code".to_string(),
    };

    let book3 = Book {
        isbn: "978-0-316-76948-0".to_string(),
        author: "Stephen King".to_string(),
        price: 1800,
        status: BookStatus::Lost,
        title: "The Shining".to_string(),
    };

    let book4 = Book {
        isbn: "978-1-4028-9462-6".to_string(),
        author: "George Orwell".to_string(),
        price: 1300,
        status: BookStatus::Available,
        title: "1984".to_string(),
    };

    let book5 = Book {
        isbn: "978-0-452-28423-4".to_string(),
        author: "F. Scott Fitzgerald".to_string(),
        price: 1700,
        status: BookStatus::Borrowed("Bob Smith".to_string()),
        title: "The Great Gatsby".to_string(),
    };

    let book6 = Book {
        isbn: "978-0-06-112008-4".to_string(),
        author: "Harper Lee".to_string(),
        price: 1600,
        status: BookStatus::Available,
        title: "To Kill a Mockingbird".to_string(),
    };

    let book7 = Book {
        isbn: "978-0-14-028333-4".to_string(),
        author: "J.R.R. Tolkien".to_string(),
        price: 2100,
        status: BookStatus::Lost,
        title: "The Hobbit".to_string(),
    };

    let books = vec![book1, book2, book3, book4, book5, book6, book7];
    books
}

pub fn input_num() -> i32 {
    input_str()
        .trim()
        .parse::<i32>()
        .expect("can't parse that.")
}

pub fn input_str() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Unable to read");
    x
}
