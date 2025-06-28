pub fn ground() {
    let x = 5;
    println!(" {x}");
    let y = x + 1;
    println!(" {y}");
}

#[allow(warnings)]
fn print_slice<T>(v: &[T]) {
    for x in v {
        // println!("{x}");
    }
}

#[allow(warnings)]
pub fn test() {
    print_slice(&[1, 2, 3]);
}
