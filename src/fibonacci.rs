use crate::my_utils;

pub fn nth_fibonacci() {
    println!("Enter n (the size of the series)");
    let mut n = my_utils::input_num();
    let mut a = 0;
    let mut b = 1;
    if n > 0 {
        print!("{} ", a);
    }
    if n > 1 {
        print!("{} ", b);
    }
    while n > 2 {
        n -= 1;
        b = b + a;
        a = b - a;
        print!("{} ", b);
    }
}
